use wasm_bindgen::prelude::*;
use iml_checker::check_arena;
use iml_syntax::{to_human_readable, from_human_readable};

#[wasm_bindgen]
pub fn validate_ast(json_str: &str) -> Result<String, JsValue> {
    // 1. (Removed conflicting iml_core v2 validator. We rely purely on the legacy v1 core_arena parse below)

    // 2. Linear DAG and ownership checking
    let core_arena: iml_core_lib::Arena = match serde_json::from_str(json_str) {
        Ok(a) => a,
        Err(e) => {
            let trace = iml_core::error::ErrorTrace {
                errors: vec![iml_core::error::RepairError::ParseError {
                    details: format!("AST compatibility error: {}", e)
                }]
            };
            return Ok(serde_json::to_string(&trace).unwrap_or_default());
        }
    };

    if let Err(e) = check_arena(&core_arena) {
        let trace = iml_core::error::ErrorTrace {
            errors: vec![iml_core::error::RepairError::ParseError {
                details: format!("Checker failed: {:?}", e)
            }]
        };
        return Ok(serde_json::to_string(&trace).unwrap_or_default());
    }

    Ok("OK".to_string())
}

#[wasm_bindgen]
pub fn translate_to_human(json_str: &str) -> Result<String, JsValue> {
    let core_arena: iml_core_lib::Arena = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(to_human_readable(&core_arena))
}

#[wasm_bindgen]
pub fn translate_from_human(human_str: &str) -> Result<String, JsValue> {
    let arena = from_human_readable(human_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let json = serde_json::to_string(&arena).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(json)
}

#[wasm_bindgen]
pub fn simulate_execution(json_str: &str, fuel_limit: u64) -> Result<String, JsValue> {
    let core_arena: iml_core_lib::Arena = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let mut fuel = fuel_limit;
    let mut trace = Vec::new();
    
    if core_arena.nodes.is_empty() {
        return Ok("[]".to_string());
    }

    let mut stack = vec![0];
    
    while let Some(idx) = stack.pop() {
        if fuel == 0 {
            trace.push("Trap: Fuel exhaustion".to_string());
            break;
        }
        fuel -= 1;

        if let Some(node) = core_arena.nodes.get(idx) {
            trace.push(format!("Step {}: executed {:?} node", idx, node.node_type));
            for &child in node.children.iter().rev() {
                stack.push(child);
            }
        } else {
            trace.push(format!("Trap: Out of bounds reference {}", idx));
            break;
        }
    }
    
    Ok(serde_json::to_string(&trace).unwrap_or_default())
}
