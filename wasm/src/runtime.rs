use wasmtime::*;

pub struct SandboxConfig {
    pub fuel_limit: u64,
    pub max_memory_bytes: usize,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            fuel_limit: 100_000_000,
            max_memory_bytes: 10 * 1024 * 1024, // 10 MB
        }
    }
}

pub struct WasmSandbox {
    engine: Engine,
    config: SandboxConfig,
}

impl WasmSandbox {
    pub fn new(config: SandboxConfig) -> anyhow::Result<Self> {
        let mut wasm_config = Config::new();
        wasm_config.consume_fuel(true);
        wasm_config.epoch_interruption(true);
        wasm_config.wasm_component_model(true);
        
        let engine = Engine::new(&wasm_config)?;
        Ok(Self { engine, config })
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }
}

// Memory limits for sandboxing
pub struct SandboxLimiter {
    max_memory: usize,
    current_memory: usize,
}

impl SandboxLimiter {
    pub fn new(max_memory: usize) -> Self {
        Self {
            max_memory,
            current_memory: 0,
        }
    }
}

impl ResourceLimiter for SandboxLimiter {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> Result<bool> {
        if desired > self.max_memory {
            return Ok(false);
        }
        self.current_memory = desired;
        Ok(true)
    }

    fn table_growing(
        &mut self,
        _current: u32,
        _desired: u32,
        _maximum: Option<u32>,
    ) -> Result<bool> {
        Ok(true)
    }
}
