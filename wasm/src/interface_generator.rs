use wasmtime::component::{Component, types::ComponentItem};
use wasmtime::Engine;
use serde_json::{json, Value};
use anyhow::Result;

pub struct UniversalInterfaceGenerator;

impl UniversalInterfaceGenerator {
    pub fn generate_schema(engine: &Engine, wasm_bytes: &[u8]) -> Result<Value> {
        let component = Component::new(engine, wasm_bytes)?;
        let mut schemas = Vec::new();

        // Introspect exports using wasmtime's native types
        let comp_ty = component.component_type();
        for (name, item) in comp_ty.exports(engine) {
            match item {
                ComponentItem::ComponentFunc(func_ty) => {
                    let mut args = Vec::new();
                    for (idx, _ty) in func_ty.params().enumerate() {
                        args.push(json!({
                            "description": format!("Argument {}", idx),
                            // In a real implementation we would map Wasm types to AST types here
                            "type": "object"
                        }));
                    }
                    
                    // Format into IML AST JSON Schema format (single character keys)
                    // We represent calling a foreign function as returning a template IML node.
                    schemas.push(json!({
                        "function_name": name,
                        "iml_template": {
                            "t": { "V": name }, // Using Var as the function identifier
                            "c": args,
                            "o": null
                        }
                    }));
                }
                ComponentItem::Resource(_) => {
                    // Expose Resource<T> lifecycles to the iml-checker linear type pass
                    schemas.push(json!({
                        "resource_name": name,
                        "lifecycle": {
                            "acquire": {
                                "t": "L", // Alloc
                                "c": [],
                                "o": format!("{}_handle", name)
                            },
                            "release": {
                                "t": "D", // Drop
                                "c": [{"t": {"V": format!("{}_handle", name)}}],
                                "o": null
                            }
                        }
                    }));
                }
                _ => {} // Other types like ComponentType, ComponentInstance
            }
        }

        Ok(json!({
            "schema_version": "1.0",
            "description": "IML AST strict single-character schema for external Wasm Component",
            "exports": schemas
        }))
    }
}
