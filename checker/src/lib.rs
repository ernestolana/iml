use core::{Arena, NodeIndex, NodeType};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum CheckerError {
    OutOfBounds(NodeIndex),
    CycleDetected(NodeIndex),
    UnconsumedResource(NodeIndex),
    DoubleConsume(NodeIndex),
}

pub fn check_arena(arena: &Arena) -> Result<(), CheckerError> {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    for i in 0..arena.nodes.len() {
        if !visited.contains(&i) {
            check_dag_and_bounds(arena, i, &mut visited, &mut stack)?;
        }
    }
    
    let mut allocations = HashSet::new();
    let mut consumed = HashSet::new();
    
    for (idx, node) in arena.nodes.iter().enumerate() {
        if let NodeType::Alloc = node.node_type {
            allocations.insert(idx);
        }
    }
    
    for node in &arena.nodes {
        if let NodeType::Drop = node.node_type {
            for &child in &node.children {
                if consumed.contains(&child) { return Err(CheckerError::DoubleConsume(child)); }
                consumed.insert(child);
            }
        }
    }
    
    for alloc in allocations {
        if !consumed.contains(&alloc) { return Err(CheckerError::UnconsumedResource(alloc)); }
    }
    Ok(())
}

fn check_dag_and_bounds(arena: &Arena, node_idx: NodeIndex, visited: &mut HashSet<NodeIndex>, stack: &mut HashSet<NodeIndex>) -> Result<(), CheckerError> {
    if node_idx >= arena.nodes.len() { return Err(CheckerError::OutOfBounds(node_idx)); }
    if stack.contains(&node_idx) { return Err(CheckerError::CycleDetected(node_idx)); }
    if visited.contains(&node_idx) { return Ok(()); }
    
    stack.insert(node_idx);
    let node = &arena.nodes[node_idx];
    for &child in &node.children {
        check_dag_and_bounds(arena, child, visited, stack)?;
    }
    stack.remove(&node_idx);
    visited.insert(node_idx);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::{Node, NodeType};
    #[test]
    fn test_valid_arena() {
        let arena = Arena { nodes: vec![
            Node { node_type: NodeType::Alloc, children: vec![], rationale: None, ownership: None },
            Node { node_type: NodeType::Drop, children: vec![0], rationale: None, ownership: None },
        ]};
        assert_eq!(check_arena(&arena), Ok(()));
    }
    #[test]
    fn test_cycle() {
        let arena = Arena { nodes: vec![
            Node { node_type: NodeType::Add, children: vec![1], rationale: None, ownership: None },
            Node { node_type: NodeType::Add, children: vec![0], rationale: None, ownership: None },
        ]};
        let err = check_arena(&arena);
        assert!(matches!(err, Err(CheckerError::CycleDetected(_))));
    }
    #[test]
    fn test_unconsumed() {
        let arena = Arena { nodes: vec![ Node { node_type: NodeType::Alloc, children: vec![], rationale: None, ownership: None } ]};
        assert_eq!(check_arena(&arena), Err(CheckerError::UnconsumedResource(0)));
    }
    #[test]
    fn test_double_consume() {
        let arena = Arena { nodes: vec![
            Node { node_type: NodeType::Alloc, children: vec![], rationale: None, ownership: None },
            Node { node_type: NodeType::Drop, children: vec![0], rationale: None, ownership: None },
            Node { node_type: NodeType::Drop, children: vec![0], rationale: None, ownership: None },
        ]};
        assert_eq!(check_arena(&arena), Err(CheckerError::DoubleConsume(0)));
    }
    #[test]
    fn test_out_of_bounds() {
        let arena = Arena { nodes: vec![ Node { node_type: NodeType::Add, children: vec![99], rationale: None, ownership: None } ]};
        assert_eq!(check_arena(&arena), Err(CheckerError::OutOfBounds(99)));
    }
}
