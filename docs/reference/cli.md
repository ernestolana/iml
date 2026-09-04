# `cli` Crate

## Commands & Arguments

### `run`
Executes target file in sandbox.
- `<file>`: Target file path.

### `format`
Serializes and deserializes target file.
- `--to-human`: Translates JSON AST to human-readable format.
- `--to-json`: Translates human-readable format to JSON AST.
- `<file>`: Target file path.

### `grammar`
Exports IML schemas.
- `--export <FORMAT>`: Output format. Accepts `json` or `gbnf`.
