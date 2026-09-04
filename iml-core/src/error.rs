use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Error)]
#[serde(tag = "error_type")]
pub enum RepairError {
    #[error("Missing mandatory 'r' (human_rationale) field at NodeIndex({index})")]
    MissingRationale { index: usize, message: String },
    
    #[error("Invalid or missing node type 't' at NodeIndex({index}). Found: {found}")]
    InvalidNodeType { index: usize, found: String, message: String },
    
    #[error("Schema validation failed at NodeIndex({index}): {details}")]
    SchemaMismatch { index: usize, details: String, message: String },
    
    #[error("Parse error: {details}")]
    ParseError { details: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorTrace {
    pub errors: Vec<RepairError>,
}
