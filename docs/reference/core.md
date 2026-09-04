# `core` Crate

## `Arena` Struct
Container for the flat AST.
- `nodes` (`Vec<Node>`): Mapped to JSON key `"n"`.

## `Node` Enum
Polymorphic node type identified by tag `"t"`. 

- `"F"` (`FunctionDef`)
  - `"r"`: `human_rationale` (`String`)
  - `"n"`: `name` (`String`)
  - `"c"`: `children` (`Vec<usize>`)

- `"P"` (`ProbabilisticLogicBlock`)
  - `"r"`: `human_rationale` (`String`)
  - `"o"`: `condition` (`String`)
  - `"p"`: `probability` (`f64`)
  - `"c"`: `children` (`Vec<usize>`)

- `"M"` (`MathematicalState`)
  - `"r"`: `human_rationale` (`String`)
  - `"e"`: `equations` (`Vec<String>`)

- `"D"` (`MemoryDrop`)
  - `"r"`: `human_rationale` (`String`)
  - `"v"`: `variables` (`Vec<String>`)
