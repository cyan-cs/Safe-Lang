// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

// Build line start offsets for faster line/column lookup.
pub fn build_line_starts(input: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (idx, ch) in input.char_indices() {
        if ch == '\n' {
            starts.push(idx + 1);
        }
    }
    starts
}

// Convert a byte offset to 1-based (line, column).
pub fn line_col_from_offset(input: &str, line_starts: &[usize], offset: usize) -> (usize, usize) {
    let offset = offset.min(input.len());
    let line_idx = match line_starts.binary_search(&offset) {
        Ok(i) => i,
        Err(i) => i.saturating_sub(1),
    };
    let line_start = line_starts
        .get(line_idx)
        .copied()
        .unwrap_or(0)
        .min(input.len());
    let column = input[line_start..offset].chars().count() + 1;
    (line_idx + 1, column)
}
