use core::{Arena, Node, NodeType};

pub fn to_human_readable(arena: &Arena) -> String {
    let mut out = String::new();
    for (idx, node) in arena.nodes.iter().enumerate() {
        let type_str = match &node.node_type {
            NodeType::Num(n) => format!("Num({})", n),
            NodeType::Add => "Add".to_string(),
            NodeType::Mul => "Mul".to_string(),
            NodeType::Alloc => "Alloc".to_string(),
            NodeType::Drop => "Drop".to_string(),
            NodeType::Var(v) => format!("Var({})", v),
            NodeType::AlgebraicMatrix(data, rows, cols) => format!("AlgebraicMatrix({}x{}, len={})", rows, cols, data.len()),
            NodeType::QuantumState(data) => format!("QuantumState(len={})", data.len()),
        };
        let children_str = if node.children.is_empty() {
            "".to_string()
        } else {
            let ch_strs: Vec<String> = node.children.iter().map(|c| c.to_string()).collect();
            format!(" -> {}", ch_strs.join(", "))
        };
        out.push_str(&format!("{}: {}{}\n", idx, type_str, children_str));
    }
    out
}

pub fn from_human_readable(s: &str) -> Result<Arena, String> {
    let mut nodes = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let mut node = Node { node_type: NodeType::Add, children: vec![], rationale: None, ownership: None };
        if line.contains("Num") { node.node_type = NodeType::Num(0); }
        else if line.contains("Alloc") { node.node_type = NodeType::Alloc; }
        else if line.contains("Drop") { node.node_type = NodeType::Drop; }
        else if line.contains("AlgebraicMatrix") { node.node_type = NodeType::AlgebraicMatrix(vec![], 0, 0); }
        else if line.contains("QuantumState") { node.node_type = NodeType::QuantumState(vec![]); }
        nodes.push(node);
    }
    Ok(Arena { nodes })
}
