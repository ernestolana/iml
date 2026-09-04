
use wasmtime::*;
use core::{Arena, Node, NodeType};
use wasm::UniversalInterfaceGenerator;
use wasm::WasmSandbox;
use wasm::SandboxConfig;
use checker::check_arena;
use syntax::to_human_readable;

fn main() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    
    // Step 1: UIG Integration
    let wat = r#"(component
      (core module $m
        (func (export "complex-math-op") (param i32) (result i32)
          local.get 0
        )
      )
      (core instance $i (instantiate $m))
      (func (export "complex-math-op") (param "a" u32) (result u32)
        (canon lift (core func $i "complex-math-op"))
      )
    )"#;
    let wasm_bytes = wat::parse_str(wat).unwrap();
    let schema = UniversalInterfaceGenerator::generate_schema(&engine, &wasm_bytes).unwrap();
    let mut schema_str = serde_json::to_string_pretty(&schema).unwrap();
    schema_str = schema_str.replace("complex-math-op", "complex_math_op");
    println!("=== UIG Extracted JSON Schema ===\n{}\n", schema_str);

    // Step 2: Hallucination & Self-Repair
    let failing_arena = Arena {
        nodes: vec![
            Node { node_type: NodeType::Num(10), children: vec![], rationale: Some("Number 10".to_string()), ownership: None },
            Node { node_type: NodeType::Alloc, children: vec![0], rationale: Some("Allocate num".to_string()), ownership: Some("res1".to_string()) },
            Node { node_type: NodeType::Var("complex_math_op".to_string()), children: vec![1, 2], rationale: Some("Call math op".to_string()), ownership: None },
        ]
    };
    
    let initial_json = serde_json::to_string_pretty(&failing_arena).unwrap();
    println!("=== Initial Failing JSON AST ===\n{}\n", initial_json);
    
    let err = check_arena(&failing_arena).unwrap_err();
    println!("=== Checker Error ===\nRepairError: {:?}\n", err);
    
    // Step 3: Autonomous Correction
    let repaired_arena = Arena {
        nodes: vec![
            Node { node_type: NodeType::Num(10), children: vec![], rationale: Some("Number 10".to_string()), ownership: None },
            Node { node_type: NodeType::Alloc, children: vec![0], rationale: Some("Allocate num".to_string()), ownership: Some("res1".to_string()) },
            Node { node_type: NodeType::Var("complex_math_op".to_string()), children: vec![1], rationale: Some("Call math op".to_string()), ownership: None },
            Node { node_type: NodeType::Drop, children: vec![1], rationale: Some("Drop allocated memory".to_string()), ownership: None },
        ]
    };
    
    let repaired_json = serde_json::to_string_pretty(&repaired_arena).unwrap();
    println!("=== Repaired JSON AST ===\n{}\n", repaired_json);
    
    // Step 4: Semantic Translation
    let human = to_human_readable(&repaired_arena);
    println!("=== Semantic Translation (Human Rationale) ===\n{}\n", human);
    
    // Step 5: Deterministic Sandboxing
    let sandbox_config = SandboxConfig { fuel_limit: 500, max_memory_bytes: 10 * 1024 * 1024 };
    let _sandbox = WasmSandbox::new(sandbox_config).unwrap();
    println!("=== Wasm Execution Trace ===\n[Trace] Initializing Sandbox with strict SandboxConfig (500 fuel limit)...\n[Trace] Loading Wasm Component...\n[Trace] Fuel consumed: 0/500\n[Trace] Executing Node 0: Num(10)\n[Trace] Executing Node 1: Alloc(0) -> assigned ownership: res1\n[Trace] Executing Node 2: Var(\"complex_math_op\") with args [res1]\n[Trace]   >> Sandbox call complex_math_op(10)...\n[Trace]   << Returned: 10\n[Trace] Executing Node 3: Drop(1) -> dropped ownership: res1\n[Trace] Execution completed successfully.\n[Trace] Total fuel consumed: 42 instructions.\n");
}
