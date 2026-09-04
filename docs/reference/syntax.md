# `syntax` Crate

## `to_human_readable` Function
`pub fn to_human_readable(arena: &Arena) -> String`

Serializes `Arena` objects mapping `NodeType` values:
- `NodeType::Num(n)` -> `Num({n})`
- `NodeType::Add` -> `Add`
- `NodeType::Mul` -> `Mul`
- `NodeType::Alloc` -> `Alloc`
- `NodeType::Drop` -> `Drop`
- `NodeType::Var(v)` -> `Var({v})`
- `NodeType::AlgebraicMatrix` -> `AlgebraicMatrix({rows}x{cols}, len={len})`
- `NodeType::QuantumState` -> `QuantumState(len={len})`

Child node pointers formatted as `-> {child_indices}`.

## `from_human_readable` Function
`pub fn from_human_readable(s: &str) -> Result<Arena, String>`

Deserializes strings back to `Arena`.
