use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::CodeGenerator;
use crate::Molder;
use crate::TypeChecker;
use crate::lexer;
use crate::parser;

const INIT_MANIFEST: &str = "name = \"safe-project\"\nversion = \"1.0\"\n";
const INIT_MAIN_SAFE: &str = "safe fn main() {\n    let high_size: usize = 4\n    let high_buf = allocate_buffer(high_size)\n    deallocate_buffer(high_buf)\n}\n";

pub fn run(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let args: Vec<String> = args.into_iter().collect();
    if args.is_empty() {
        return Err(usage());
    }

    match args[0].as_str() {
        "build" => {
            if args.len() != 2 {
                return Err("Usage: safe build <file.safe>".to_string());
            }
            build_command(Path::new(&args[1]))
        }
        "init" => {
            if args.len() == 1 {
                init_current_dir()
            } else if args.len() == 2 {
                init_new_project(&args[1])
            } else {
                Err("Usage: safe init [project-name]".to_string())
            }
        }
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "Usage:\n  safe build <file.safe>\n  safe init\n  safe init <project-name>".to_string()
}

fn build_command(entry_file: &Path) -> Result<(), String> {
    let entry_abs = canonicalize_existing(entry_file)?;
    let merged_source = collect_source_with_imports(&entry_abs)?;
    let generated = compile_to_rust(&merged_source)?;

    let output_path = entry_abs.with_extension("rs");
    fs::write(&output_path, generated)
        .map_err(|e| format!("Failed to write output '{}': {}", output_path.display(), e))?;

    println!("Build successful: {}", output_path.display());
    Ok(())
}

fn compile_to_rust(input: &str) -> Result<String, String> {
    let tokens = lexer::tokenize(input).map_err(|e| format!("Lexing failed: {e}"))?;
    let (rest, source) = parser::parse(&tokens).map_err(|e| format!("Parsing failed: {e:?}"))?;
    if !rest.is_empty() {
        return Err("Parsing failed: unconsumed tokens".to_string());
    }

    let mut molder = Molder::new(source);
    molder.mold().map_err(|e| format!("Molding failed: {e}"))?;

    let mut checker = TypeChecker::new();
    checker
        .check(molder.get_output())
        .map_err(|e| format!("Type checking failed: {e}"))?;

    let mut generator = CodeGenerator::new();
    generator
        .generate(molder.get_output())
        .map_err(|e| format!("Code generation failed: {e}"))
}

fn collect_source_with_imports(entry_file: &Path) -> Result<String, String> {
    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();
    let mut stack = Vec::new();
    let mut cache = HashMap::new();
    let mut output = String::new();

    collect_recursive(
        entry_file,
        &mut visited,
        &mut visiting,
        &mut stack,
        &mut cache,
        &mut output,
    )?;

    Ok(output)
}

fn collect_recursive(
    file: &Path,
    visited: &mut HashSet<PathBuf>,
    visiting: &mut HashSet<PathBuf>,
    stack: &mut Vec<PathBuf>,
    cache: &mut HashMap<PathBuf, String>,
    output: &mut String,
) -> Result<(), String> {
    let canonical = canonicalize_existing(file)?;

    if visited.contains(&canonical) {
        return Ok(());
    }

    if visiting.contains(&canonical) {
        let mut chain = stack
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>();
        chain.push(canonical.display().to_string());
        return Err(format!("Import cycle detected: {}", chain.join(" -> ")));
    }

    visiting.insert(canonical.clone());
    stack.push(canonical.clone());

    let content = if let Some(cached) = cache.get(&canonical) {
        cached.clone()
    } else {
        let loaded = fs::read_to_string(&canonical)
            .map_err(|e| format!("Failed to read '{}': {}", canonical.display(), e))?;
        cache.insert(canonical.clone(), loaded.clone());
        loaded
    };

    let parent = canonical
        .parent()
        .ok_or_else(|| format!("Invalid parent path: {}", canonical.display()))?;
    let mut body = String::new();

    for line in content.lines() {
        if let Some(import_path) = parse_import_line(line) {
            let import_full = parent.join(import_path);
            collect_recursive(&import_full, visited, visiting, stack, cache, output)?;
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }

    output.push_str(&body);
    if !body.ends_with('\n') {
        output.push('\n');
    }

    stack.pop();
    visiting.remove(&canonical);
    visited.insert(canonical);
    Ok(())
}

fn parse_import_line(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if !trimmed.starts_with("import") {
        return None;
    }

    let rest = trimmed.strip_prefix("import")?.trim_start();
    if !(rest.starts_with('"') && rest.ends_with('"') && rest.len() >= 2) {
        return None;
    }
    Some(&rest[1..rest.len() - 1])
}

fn init_current_dir() -> Result<(), String> {
    let cwd = env::current_dir().map_err(|e| format!("Failed to get current dir: {e}"))?;
    init_project_at(&cwd, false)
}

fn init_new_project(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Project name cannot be empty".to_string());
    }
    let cwd = env::current_dir().map_err(|e| format!("Failed to get current dir: {e}"))?;
    let project_dir = cwd.join(name);
    init_project_at(&project_dir, true)
}

fn init_project_at(path: &Path, create_root: bool) -> Result<(), String> {
    if create_root {
        if path.exists() {
            return Err(format!("Directory already exists: {}", path.display()));
        }
        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory '{}': {}", path.display(), e))?;
    } else if !path.exists() {
        return Err(format!("Directory does not exist: {}", path.display()));
    }

    let src_dir = path.join("src");
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("Failed to create '{}': {}", src_dir.display(), e))?;
    }

    let manifest_path = path.join("Safe.toml");
    let main_safe_path = src_dir.join("main.safe");

    write_if_missing(&manifest_path, INIT_MANIFEST)?;
    write_if_missing(&main_safe_path, INIT_MAIN_SAFE)?;

    println!("Initialized SAFE project at {}", path.display());
    Ok(())
}

fn write_if_missing(path: &Path, content: &str) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }
    fs::write(path, content).map_err(|e| format!("Failed to write '{}': {}", path.display(), e))
}

fn canonicalize_existing(path: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(path).map_err(|e| format!("Path not found '{}': {}", path.display(), e))
}

#[cfg(test)]
mod tests {
    use super::{collect_source_with_imports, parse_import_line};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let dir = env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), now));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn test_parse_import_line() {
        assert_eq!(parse_import_line("import \"a.safe\""), Some("a.safe"));
        assert_eq!(
            parse_import_line("  import   \"dir/b.safe\"  "),
            Some("dir/b.safe")
        );
        assert_eq!(parse_import_line("import a.safe"), None);
        assert_eq!(parse_import_line("let high_x = 1"), None);
    }

    #[test]
    fn test_collect_source_with_imports_merges_dependencies() {
        let dir = temp_dir("safe_import_merge");
        let entry = dir.join("file1.safe");
        let dep = dir.join("file2.safe");

        fs::write(
            &dep,
            "safe fn dep() {\n    let high_size: usize = 1\n    let high_buf = allocate_buffer(high_size)\n    deallocate_buffer(high_buf)\n}\n",
        )
        .expect("write dep");
        fs::write(
            &entry,
            "import \"file2.safe\"\nsafe fn main() {\n    dep()\n}\n",
        )
        .expect("write entry");

        let merged = collect_source_with_imports(&entry).expect("collect");
        assert!(merged.contains("safe fn dep()"));
        assert!(merged.contains("safe fn main()"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_source_with_imports_detects_cycle() {
        let dir = temp_dir("safe_import_cycle");
        let f1 = dir.join("file1.safe");
        let f2 = dir.join("file2.safe");

        fs::write(&f1, "import \"file2.safe\"\nsafe fn a() {}\n").expect("write f1");
        fs::write(&f2, "import \"file1.safe\"\nsafe fn b() {}\n").expect("write f2");

        let err = collect_source_with_imports(&f1).expect_err("cycle should fail");
        assert!(err.contains("Import cycle detected"));

        let _ = fs::remove_dir_all(&dir);
    }
}
