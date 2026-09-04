use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Arena {
    #[serde(rename = "n")]
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", deny_unknown_fields)]
pub enum Node {
    #[serde(rename = "F")]
    FunctionDef {
        #[serde(rename = "r")]
        human_rationale: String,
        #[serde(rename = "n")]
        name: String,
        #[serde(rename = "c")]
        children: Vec<usize>,
    },
    
    #[serde(rename = "P")]
    ProbabilisticLogicBlock {
        #[serde(rename = "r")]
        human_rationale: String,
        #[serde(rename = "o")]
        condition: String,
        #[serde(rename = "p")]
        probability: f64,
        #[serde(rename = "c")]
        children: Vec<usize>,
    },
    
    #[serde(rename = "M")]
    MathematicalState {
        #[serde(rename = "r")]
        human_rationale: String,
        #[serde(rename = "e")]
        equations: Vec<String>,
    },
    
    #[serde(rename = "D")]
    MemoryDrop {
        #[serde(rename = "r")]
        human_rationale: String,
        #[serde(rename = "v")]
        variables: Vec<String>,
    },
}
