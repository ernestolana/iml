
with open('iml-web/src/lib.rs', 'r', encoding='utf-8') as f:
    text = f.read()

import re

old_sim = r'''#\[wasm_bindgen\]\s*pub fn simulate_execution.*?Ok\(serde_json::to_string\(&trace\).*?\)[\s\r\n]*\}'''

new_sim = '''#[wasm_bindgen]
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
}'''

new_text = re.sub(old_sim, new_sim, text, flags=re.DOTALL)

with open('iml-web/src/lib.rs', 'w', encoding='utf-8') as f:
    f.write(new_text)
