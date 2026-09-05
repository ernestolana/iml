import json
import tiktoken

enc = tiktoken.get_encoding("cl100k_base")

def get_metrics(text):
    return {
        "tokens": len(enc.encode(text)),
        "bytes": len(text.encode('utf-8'))
    }

python_code = """import numpy as np
from solver import WasmSolver

def stabilize_power(grid_state):
    solver = WasmSolver.allocate()
    try:
        tensor = np.array(grid_state).reshape((2, 2))
        return solver.multiply(tensor)
    finally:
        solver.free()"""

rust_code = """use ndarray::Array2;
use solver::WasmSolver;

fn stabilize_power(grid_state: Vec<f64>) -> Result<Array2<f64>, Error> {
    let solver = WasmSolver::allocate()?;
    let tensor = Array2::from_shape_vec((2, 2), grid_state)?;
    let result = solver.multiply(&tensor);
    drop(solver);
    Ok(result)
}"""

verbose_json_code = """{
  "type": "Program",
  "body": [
    {
      "type": "VariableDeclaration",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": { "type": "Identifier", "name": "solver" },
          "init": {
            "type": "CallExpression",
            "callee": { "type": "Identifier", "name": "allocate_solver" },
            "arguments": []
          }
        }
      ],
      "kind": "let"
    },
    {
      "type": "VariableDeclaration",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": { "type": "Identifier", "name": "tensor" },
          "init": {
            "type": "CallExpression",
            "callee": { "type": "Identifier", "name": "AlgebraicMatrix" },
            "arguments": [
              {
                "type": "ArrayExpression",
                "elements": [
                  { "type": "Literal", "value": 1.2 },
                  { "type": "Literal", "value": 0.5 },
                  { "type": "Literal", "value": 0.9 },
                  { "type": "Literal", "value": 1.1 }
                ]
              },
              { "type": "Literal", "value": 2 },
              { "type": "Literal", "value": 2 }
            ]
          }
        }
      ],
      "kind": "let"
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": { "type": "Identifier", "name": "multiply" },
        "arguments": [
          { "type": "Identifier", "name": "solver" },
          { "type": "Identifier", "name": "tensor" }
        ]
      }
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": { "type": "Identifier", "name": "drop" },
        "arguments": [
          { "type": "Identifier", "name": "solver" }
        ]
      }
    }
  ]
}"""

slop_code = """(def-stabilizer microgrid
  (bind solver (alloc-wasm-solver))
  (bind tensor (matrix 2 2 (list 1.2 0.5 0.9 1.1)))
  (bind result (mul solver tensor))
  (drop solver)
  result)"""

comparisons = {
    "python": {
        "name": "Python 3.12",
        "description": "Verbose, human-centric logic using standard mathematical libraries.",
        "code": python_code,
        "metrics": get_metrics(python_code)
    },
    "rust": {
        "name": "Rust",
        "description": "Strict, compiled logic with explicit memory handling.",
        "code": rust_code,
        "metrics": get_metrics(rust_code)
    },
    "verbose_json": {
        "name": "Verbose JSON AST",
        "description": "A deeply nested recursive JSON tree simulating a standard Agent framework target.",
        "code": verbose_json_code,
        "metrics": get_metrics(verbose_json_code)
    },
    "slop": {
        "name": "SLOP / S-Expressions",
        "description": "S-expressions mirroring Lisp for minimal parsing, but often suffering from agentic hallucinations.",
        "code": slop_code,
        "metrics": get_metrics(slop_code)
    }
}

import os
os.makedirs('static-site/src/data', exist_ok=True)
with open('static-site/src/data/comparisons.json', 'w', encoding='utf-8') as f:
    json.dump(comparisons, f, indent=2)

print("Created static-site/src/data/comparisons.json")
