# Intelligent Machine Language (IML)

IML is a programming language optimized exclusively for LLMs and AI agents, eschewing human syntax in favor of token-dense structural primitives.

## Architecture

- **Core (`iml-core`)**: Provides the flat, index-based `Arena` AST structure. This architecture avoids deep JSON nesting that typically breaks LLM context generation. It uses single-character Serde tags (`t`, `c`, `r`, `o`) for maximum token density, and natively supports complex types like `AlgebraicMatrix` and `QuantumState`.
- **Checker (`iml-checker`)**: A two-pass linear-type and bounds checker. Validates that every allocated resource is strictly consumed and prevents AST cycles.
- **Syntax (`iml-syntax`)**: A bidirectional semantic overlay translation layer allowing humans to read and manipulate the AST in a human-readable format.
- **Wasm (`iml-wasm`)**: A secure sandboxing environment built on `wasmtime`.

## WebAssembly Constraints

The execution runtime is severely constrained for safe agent execution:
- **Memory Epoch Limits**: Enforced using `epoch_interruption`.
- **Fuel Consumption**: A strict instruction fuel cap is set on execution to terminate infinite loops automatically.

## CLI Commands

The `iml` CLI provides tools for running and interacting with the AST.

```bash
# Run a file in the Wasm sandbox
iml run <file>

# Translate an IML JSON AST to human-readable format
iml format --to-human <file>

# Translate human-readable format back to IML JSON AST
iml format --to-json <file>

# Export the exact grammar schema for LLM constrained decoding
iml grammar --export json
iml grammar --export gbnf
```
