# Intelligent Machine Language (IML)

## AI-Native Philosophy
IML is a programming language engineered from the ground up for LLMs and AI agents. It completely discards traditional human-centric lexing, parsing, and syntax in favor of token-dense structural primitives. The entire architecture is optimized for context window efficiency, zero-hallucination semantic parsing, and autonomous agent-to-agent communication.

## Ultra-Terse JSON Schema & Dual-State Architecture
IML operates on a dual-state representation:
1. **Machine State**: A highly compressed, flat, index-based `Arena` AST encoded in an ultra-terse JSON schema. It exclusively utilizes single-character keys—such as `"t"` (type), `"c"` (children), `"r"` (reference/resource), and `"o"` (operation)—to achieve maximum token density and allow LLMs to output massive ASTs without hitting context limits.
2. **Semantic Overlay (Svelte-based)**: For human engineers, IML provides a bidirectional semantic overlay powered by a modern Svelte frontend (`iml-translator`). This layer losslessly translates the ultra-terse JSON into readable, editable natural language blocks that can be manipulated and synchronized back to the machine state.

## Two-Pass Linear Type Checker
Because autoregressive models can occasionally hallucinate invalid references or cyclic dependencies, IML employs a strict **Two-Pass Linear Type Checker** (`iml-checker`).
- **Pass 1 (Bounds & Cycle)**: Validates that all index references (`"r"`) point to valid memory locations and proves the graph is cycle-free.
- **Pass 2 (Resource Lifecycle)**: Enforces linear typing rules, ensuring that every allocated resource or complex type is consumed exactly once, guaranteeing memory safety before execution.

## Wasm Component Model UIG (Universal Interface Generator)
To safely execute agent-generated code, IML relies on a secure `wasmtime` execution sandbox enhanced by the **Wasm Component Model UIG (Universal Interface Generator)**.
The UIG automatically synthesizes WebAssembly Component Model interfaces (`.wit`) directly from the IML AST structure. This provides strongly-typed, memory-safe boundaries between the AI agent's logic and the host system, ensuring zero-trust isolated execution.

## CLI Usage & Examples

The `iml` CLI handles formatting, validation, schema extraction, and secure execution.

```bash
# 1. Translate an ultra-terse IML JSON AST into the human-readable Semantic Overlay
iml format --to-human src/agent_logic.json

# 2. Extract the strict JSON Schema for LLM constrained decoding (e.g., OpenAI Structured Outputs)
iml grammar --export json > schema.json

# 3. Execute a validated .iml file within the heavily constrained wasmtime sandbox
iml run --sandbox src/agent_logic.json
```
