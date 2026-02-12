# SAFE? v1.0 リリースノート（日本語）

公開日: 2026-02-12

## 概要
SAFE? v1.0 は、`Raw -> Validated -> High` 境界を中核に固定した初回安定版です。  
「安全をデフォルトにし、低水準操作は明示して局所化する」方針を、言語処理系・型検査・実行時ライブラリ・CLIで一貫させています。

## v1.0 の主な内容
1. CLI
- `safe build <file.safe>`: import 展開込みでトランスパイル
- `safe init`: 既存ディレクトリをプロジェクト初期化
- `safe init <project-name>`: 新規プロジェクト作成 + 初期化

2. import 解決
- `import "x.safe"` を再帰的に解決
- 依存サイクル検出を実装（無限ループ防止）

3. 言語基本構文
- 変数: `let`, `const`
- 制御構文: `if / else`, `for`, `break`, `continue`
- コメント: `//`, `/* ... */`
- 文字列:
  - 通常文字列は改行を直接含まない
  - `\n` エスケープ対応
  - raw string (`r#"..."#`) 対応

4. 出力関数
- `print(...)`: 改行なし
- `printl(...)`: 末尾に改行追加
- 複数引数対応
- 文字列/整数/真偽値などの自動文字列化

5. 安全モデルの固定
- `unsafe` ブロック外で raw 操作を許可しない
- 命名規則 (`raw_`, `validated_`, `high_`) と遷移規則を Molding + TypeChecker で検証

6. ランタイム標準型
- `core::types::String`
- `core::types::List`
- 最小 ADT: `Option<T>`, `Result<T, E>`（v1.0 実装範囲あり）

## 互換性ポリシー（v1.x）
- v1.0 の安全境界は凍結対象です。
- v1.x は後方互換を重視した追加中心とし、境界仕様を変える変更はメジャーバージョン対象です。

## 既知の制約（v1.0）
- 高度なジェネリクス・borrow/lifetime/effect system は対象外
- 高度なパターンマッチング（`match`, `if let`）は対象外

## 参照
- 安全モデル: `docs/ja/safety_model.md`
- 型システム: `docs/ja/type_system.md`
- Molding 仕様: `docs/ja/molding.md`
- 例: `examples/README.md`
