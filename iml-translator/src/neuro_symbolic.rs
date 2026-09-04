use iml_core_lib::Arena;
use checker::{check_arena, CheckerError};

pub fn validate_update(arena: &Arena) -> Result<(), CheckerError> {
    check_arena(arena)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iml_core_lib::{Node, NodeType};

    #[test]
    fn test_valid_update() {
        let arena = Arena {
            nodes: vec![
                Node {
                    node_type: NodeType::Alloc,
                    children: vec![],
                    rationale: None,
                    ownership: None,
                },
                Node {
                    node_type: NodeType::Drop,
                    children: vec![0],
                    rationale: None,
                    ownership: None,
                },
            ],
        };
        assert_eq!(validate_update(&arena), Ok(()));
    }

    #[test]
    fn test_invalid_update_cycle() {
        let arena = Arena {
            nodes: vec![
                Node {
                    node_type: NodeType::Add,
                    children: vec![1],
                    rationale: None,
                    ownership: None,
                },
                Node {
                    node_type: NodeType::Add,
                    children: vec![0],
                    rationale: None,
                    ownership: None,
                },
            ],
        };
        assert!(matches!(validate_update(&arena), Err(CheckerError::CycleDetected(_))));
    }
}
