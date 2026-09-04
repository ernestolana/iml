use wasm_bindgen::prelude::*;
use iml_core::validator::validate_and_parse;
use checker::check_arena;
use syntax::{to_human_readable, from_human_readable};

#[wasm_bindgen]
pub fn validate_ast(json_str: &str) -> Result<String, JsValue> {
    // 1. Core structural/schema validation
    if let Err(trace) = validate_and_parse(json_str) {
        return Ok(serde_json::to_string(&trace).unwrap_or_default());
    }

    // 2. Linear DAG and ownership checking
    let core_arena: core::Arena = match serde_json::from_str(json_str) {
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
    let core_arena: core::Arena = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
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
    let core_arena: core::Arena = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let mut fuel = fuel_limit;
    let mut trace = Vec::new();
    
    for (i, node) in core_arena.nodes.iter().enumerate() {
        if fuel == 0 {
            return Err(JsValue::from_str("Trap: Fuel exhaustion"));
        }
        fuel -= 1;
        trace.push(format!("Step {}: executed {:?} node", i, node.node_type));
    }
    
    Ok(serde_json::to_string(&trace).map_err(|e| JsValue::from_str(&e.to_string()))?)
}
