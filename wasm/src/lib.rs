use wasmtime::*;
use core::Arena;

pub struct WasmSandbox {
    engine: Engine,
}

impl WasmSandbox {
    pub fn new() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.consume_fuel(true);
        config.epoch_interruption(true);
        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }

    pub fn run_arena(&self, _arena: &Arena, wasm_bytes: &[u8], fuel_cap: u64) -> anyhow::Result<i32> {
        let mut store = Store::new(&self.engine, ());
        store.set_fuel(fuel_cap)?;
        store.epoch_deadline_trap();
        
        let module = Module::new(&self.engine, wasm_bytes)?;
        let instance = Instance::new(&mut store, &module, &[])?;
        let run = instance.get_typed_func::<(), i32>(&mut store, "run")?;
        let res = run.call(&mut store, ())?;
        Ok(res)
    }
}
