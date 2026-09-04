use crate::ast::{Arena, Node};
use crate::error::{ErrorTrace, RepairError};
use serde_json::Value;

pub fn validate_and_parse(json_str: &str) -> Result<Arena, ErrorTrace> {
    let value: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            return Err(ErrorTrace {
                errors: vec![RepairError::ParseError {
                    details: e.to_string(),
                }],
            });
        }
    };

    let mut errors = Vec::new();

    if let Value::Object(root) = &value {
        if let Some(Value::Array(nodes)) = root.get("n") {
            for (i, node_val) in nodes.iter().enumerate() {
                validate_node(node_val, i, &mut errors);
                
                // If the base checks pass, ensure strict schema deserialization works for this node
                // This lets us report SchemaMismatch with the exact arena index!
                if let Err(e) = serde_json::from_value::<Node>(node_val.clone()) {
                    // Only push SchemaMismatch if we haven't already reported a base structural error for this node
                    // to prevent spamming the LLM with duplicate overlapping errors
                    if !errors.iter().any(|err| match err {
                        RepairError::MissingRationale { index, .. } => *index == i,
                        RepairError::InvalidNodeType { index, .. } => *index == i,
                        _ => false,
                    }) {
                        errors.push(RepairError::SchemaMismatch {
                            index: i,
                            details: e.to_string(),
                            message: "Strict schema deserialization rejected this node's fields.".to_string(),
                        });
                    }
                }
            }
        } else {
            errors.push(RepairError::ParseError {
                details: "Arena must contain an 'n' field with an array of nodes.".to_string(),
            });
        }
    } else {
        errors.push(RepairError::ParseError {
            details: "Root of AST must be an Arena object.".to_string(),
        });
    }

    if !errors.is_empty() {
        return Err(ErrorTrace { errors });
    }

    match serde_json::from_value::<Arena>(value) {
        Ok(arena) => Ok(arena),
        Err(e) => {
            Err(ErrorTrace {
                errors: vec![RepairError::ParseError {
                    details: format!("Arena-level validation failed: {}", e),
                }]
            })
        }
    }
}

fn validate_node(val: &Value, index: usize, errors: &mut Vec<RepairError>) {
    if let Value::Object(obj) = val {
        // 1. Check human_rationale 'r'
        if !obj.contains_key("r") || !obj["r"].is_string() {
            errors.push(RepairError::MissingRationale {
                index,
                message: "A mandatory 'r' (human_rationale) string field is missing.".to_string(),
            });
        }

        // 2. Check type 't'
        let valid_types = ["F", "P", "M", "D"];
        if let Some(type_val) = obj.get("t") {
            if let Value::String(type_str) = type_val {
                if !valid_types.contains(&type_str.as_str()) {
                    errors.push(RepairError::InvalidNodeType {
                        index,
                        found: type_str.clone(),
                        message: format!("Node type '{}' is invalid. Valid types: {:?}", type_str, valid_types),
                    });
                }
            } else {
                errors.push(RepairError::InvalidNodeType {
                    index,
                    found: type_val.to_string(),
                    message: "'t' field must be a string.".to_string(),
                });
            }
        } else {
            errors.push(RepairError::InvalidNodeType {
                index,
                found: "null/missing".to_string(),
                message: "A mandatory 't' field is missing.".to_string(),
            });
        }
    } else {
        errors.push(RepairError::SchemaMismatch {
            index,
            details: "Node must be an object".to_string(),
            message: "Expected a JSON object for the AST node.".to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_arena() {
        let json = r#"{
            "n": [
                {
                    "t": "F",
                    "r": "Entry point",
                    "n": "main",
                    "c": [1]
                },
                {
                    "t": "D",
                    "r": "Free memory",
                    "v": ["temp_var"]
                }
            ]
        }"#;
        let res = validate_and_parse(json);
        assert!(res.is_ok());
    }

    #[test]
    fn test_missing_rationale() {
        let json = r#"{
            "n": [
                {
                    "t": "F",
                    "n": "main",
                    "c": []
                }
            ]
        }"#;
        let res = validate_and_parse(json);
        assert!(res.is_err());
        let errs = res.unwrap_err().errors;
        assert_eq!(errs.len(), 1);
        if let RepairError::MissingRationale { index, .. } = &errs[0] {
            assert_eq!(*index, 0);
        } else {
            panic!("Expected MissingRationale");
        }
    }

    #[test]
    fn test_invalid_node_type() {
        let json = r#"{
            "n": [
                {
                    "t": "Invalid",
                    "r": "Entry point",
                    "n": "main",
                    "c": []
                }
            ]
        }"#;
        let res = validate_and_parse(json);
        assert!(res.is_err());
        let errs = res.unwrap_err().errors;
        assert_eq!(errs.len(), 1);
        if let RepairError::InvalidNodeType { index, found, .. } = &errs[0] {
            assert_eq!(*index, 0);
            assert_eq!(found, "Invalid");
        } else {
            panic!("Expected InvalidNodeType");
        }
    }
    
    #[test]
    fn test_deny_unknown_fields() {
        let json = r#"{
            "n": [
                {
                    "t": "M",
                    "r": "Define math state",
                    "e": ["y = mx + c"],
                    "hallucinated_field": 42
                }
            ]
        }"#;
        let res = validate_and_parse(json);
        assert!(res.is_err());
        let errs = res.unwrap_err().errors;
        assert_eq!(errs.len(), 1);
        if let RepairError::SchemaMismatch { index, .. } = &errs[0] {
            assert_eq!(*index, 0);
        } else {
            panic!("Expected SchemaMismatch");
        }
    }

    #[test]
    fn test_valid_prob_logic_block() {
        let json = r#"{
            "n": [
                {
                    "t": "P",
                    "r": "If we get heads",
                    "o": "coin_flip == 1",
                    "p": 0.5,
                    "c": []
                }
            ]
        }"#;
        assert!(validate_and_parse(json).is_ok());
    }
}
