# How-to: Manage Sandbox Fuel

This guide shows you how to restrict execution time in IML by configuring fuel limits within the Wasmtime execution sandbox. 

## Rust Configuration

By default, the IML sandbox allocates `100_000_000` fuel units per execution. You can override this deterministically to prevent infinite loops from malicious or hallucinated agent code.

Use the `SandboxConfig` struct from the `iml_wasm` crate:

```rust
use iml_wasm::runtime::{SandboxConfig, WasmSandbox};

fn main() -> anyhow::Result<()> {
    // 1. Configure custom fuel limits
    let config = SandboxConfig {
        fuel_limit: 50_000, // Set fuel lower than the default
        ..Default::default()
    };

    // 2. Initialize the sandbox
    // The WasmSandbox automatically configures `wasm_config.consume_fuel(true)`
    let sandbox = WasmSandbox::new(config)?;

    // You can now execute modules securely within this engine
    let engine = sandbox.engine();
    
    Ok(())
}
```

When the executing IML code consumes all `50_000` fuel units, Wasmtime will immediately trap and halt execution.
