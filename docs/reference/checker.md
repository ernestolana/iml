# `checker` Crate

## `CheckerError` Enum
- `OutOfBounds(NodeIndex)`
- `CycleDetected(NodeIndex)`
- `UnconsumedResource(NodeIndex)`
- `DoubleConsume(NodeIndex)`

## `check_arena` Function
`pub fn check_arena(arena: &Arena) -> Result<(), CheckerError>`

Validation passes:
1. **DAG and Bounds**: Ensures all index references point to valid memory locations (`< arena.nodes.len()`) and checks the graph is cycle-free via DFS stack tracking.
2. **Resource Lifecycle**: Enforces linear typing rules. Identifies all `NodeType::Alloc` elements and ensures they are consumed exactly once by the children of a `NodeType::Drop` element.
