use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type NodeIndex = usize;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum NodeType {
    #[serde(rename = "N")] Num(i64),
    #[serde(rename = "A")] Add,
    #[serde(rename = "M")] Mul,
    #[serde(rename = "L")] Alloc,
    #[serde(rename = "D")] Drop,
    #[serde(rename = "V")] Var(String),
    #[serde(rename = "B")] AlgebraicMatrix(Vec<f64>, usize, usize),
    #[serde(rename = "Q")] QuantumState(Vec<f64>),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Node {
    #[serde(rename = "t")] pub node_type: NodeType,
    #[serde(rename = "c", default, skip_serializing_if = "Vec::is_empty")] pub children: Vec<NodeIndex>,
    #[serde(rename = "r", default, skip_serializing_if = "Option::is_none")] pub rationale: Option<String>,
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")] pub ownership: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Arena {
    pub nodes: Vec<Node>,
}

pub fn format_parse_error(err: serde_json::Error) -> String {
    format!("Syntax Error: IML expects a strict JSON schema for the AST. Your output failed to parse at line {}, column {}. Detail: {}. Please ensure you only use single-character keys ('t', 'c', 'r', 'o') and valid node types.", err.line(), err.column(), err)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serialization() {
        let arena = Arena { nodes: vec![
            Node { node_type: NodeType::Num(42), children: vec![], rationale: Some("A constant".to_string()), ownership: None },
            Node { node_type: NodeType::Alloc, children: vec![0], rationale: None, ownership: Some("res1".to_string()) }
        ]};
        let json = serde_json::to_string(&arena).unwrap();
        assert!(json.contains("\"t\":{\"N\":42}"));
        let deserialized: Arena = serde_json::from_str(&json).unwrap();
        assert_eq!(arena, deserialized);
    }
}
pub mod gbnf;
