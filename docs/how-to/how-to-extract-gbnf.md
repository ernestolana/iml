# How-to: Extract GBNF for LLM Constrained Decoding

This guide shows you how to extract the IML structural grammar into GBNF (GGML BNF) format. GBNF is commonly used with tools like `llama.cpp` to force the LLM to output valid IML ASTs.

## Via CLI

To instantly generate the GBNF file from the terminal, use the `iml grammar` command:

```bash
iml grammar --export gbnf > grammar.gbnf
```

You can then pass this file directly into your LLM runtime (e.g., `llama.cpp --grammar-file grammar.gbnf`).

## Via Rust

To extract GBNF programmatically within an agent wrapper or custom backend, use the `iml_core` crate.

```rust
use iml_core::gbnf::schema_to_gbnf;
use iml_core::schema::generate_schema;

fn main() {
    // 1. Generate the IML Schema
    let schema = generate_schema();
    
    // 2. Convert to GBNF
    let gbnf = schema_to_gbnf(&schema);
    
    // 3. Output or pass to your LLM API
    println!("{}", gbnf);
}
```
