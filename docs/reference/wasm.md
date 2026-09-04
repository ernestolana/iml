# `wasm` Crate

## `SandboxConfig` Struct
- `fuel_limit` (`u64`): Default `100_000_000`.
- `max_memory_bytes` (`usize`): Default `10,485,760` (10 MB).

## `WasmSandbox` Struct
Initializes `wasmtime::Engine` with:
- `wasm_config.consume_fuel(true)`
- `wasm_config.epoch_interruption(true)`
- `wasm_config.wasm_component_model(true)`

## `SandboxLimiter` Struct
Implements `wasmtime::ResourceLimiter`.
- Caps memory at `max_memory_bytes`.
- Rejects memory allocations exceeding max.

## `UniversalInterfaceGenerator`
`pub fn generate_schema(engine: &Engine, wasm_bytes: &[u8]) -> Result<Value>`

### Wasm Component Mappings
Maps Wasm component exports to IML JSON schema:
- **Component Functions (`ComponentItem::ComponentFunc`)**:
  - `t`: `{ "V": name }`
  - `c`: `[ ...args (type: "object") ]`
  - `o`: `null`
- **Component Resources (`ComponentItem::Resource`)**:
  - `acquire`:
    - `t`: `"L"` (Alloc)
    - `c`: `[]`
    - `o`: `"{name}_handle"`
  - `release`:
    - `t`: `"D"` (Drop)
    - `c`: `[{"t": {"V": "{name}_handle"}}]`
    - `o`: `null`
