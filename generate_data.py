import json

iml_code = """{
  "nodes": [
    {"t":{"L":null}, "c":[], "r":"Allocate thermodynamic power grid solver component from the host environment."},
    {"t":{"B":[[1.2,0.5,0.9,1.1],2,2]}, "c":[], "r":"Initialize 2x2 grid state tensor with baseline power and frequency metrics."},
    {"t":{"M":null}, "c":[0,1], "r":"Execute matrix stabilization delta calculation."},
    {"t":{"D":null}, "c":[0], "r":"Explicitly drop thermodynamic solver handle to prevent memory leaks."}
  ]
}"""

data = {
  'iml': {
    'ast': iml_code,
    'overlay': [
        "Step 1: Allocate thermodynamic power grid solver component from the host environment.",
        "Step 2: Initialize 2x2 grid state tensor with baseline power and frequency metrics.",
        "Step 3: Execute matrix stabilization delta calculation.",
        "Step 4: Explicitly drop thermodynamic solver handle to prevent memory leaks."
    ],
    'trace': "Step 0: executed Alloc node\nStep 1: executed AlgebraicMatrix([1.2, 0.5, 0.9, 1.1], 2, 2) node\nStep 2: executed Mul node (Stabilization delta calculated)\nStep 3: executed Drop node\n\n[SUCCESS] Execution complete. Final stabilization delta applied. (Fuel remaining: 96)"
  },
  'comparisons': {
    'python': {
      'name': 'Python 3.12',
      'code': "import numpy as np\nfrom solver import WasmSolver\n\ndef stabilize_power(grid_state):\n    solver = WasmSolver.allocate()\n    try:\n        tensor = np.array(grid_state).reshape((2, 2))\n        return solver.multiply(tensor)\n    finally:\n        solver.free()",
      'metrics': {'tokens': 78, 'bytes': 254},
      'risk': 'High (Syntax Errors, Indentation, Missing imports, Unclosed resources)'
    },
    'rust': {
      'name': 'Rust',
      'code': "use ndarray::Array2;\nuse solver::WasmSolver;\n\nfn stabilize_power(grid_state: Vec<f64>) -> Result<Array2<f64>, Error> {\n    let solver = WasmSolver::allocate()?;\n    let tensor = Array2::from_shape_vec((2, 2), grid_state)?;\n    let result = solver.multiply(&tensor);\n    drop(solver);\n    Ok(result)\n}",
      'metrics': {'tokens': 94, 'bytes': 310},
      'risk': 'Medium (Borrow checker failures, lifetime mismatches, strict types)'
    },
    'verbose_json': {
      'name': 'Verbose JSON AST',
      'code': "{\n  \"type\": \"Program\",\n  \"body\": [\n    { \"type\": \"VariableDeclaration\", \"declarations\": [ { \"type\": \"VariableDeclarator\", \"id\": { \"type\": \"Identifier\", \"name\": \"solver\" }, \"init\": { \"type\": \"CallExpression\", \"callee\": { \"type\": \"Identifier\", \"name\": \"allocate_solver\" }, \"arguments\": [] } } ], \"kind\": \"let\" },\n    { \"type\": \"VariableDeclaration\", \"declarations\": [ { \"type\": \"VariableDeclarator\", \"id\": { \"type\": \"Identifier\", \"name\": \"tensor\" }, \"init\": { \"type\": \"CallExpression\", \"callee\": { \"type\": \"Identifier\", \"name\": \"AlgebraicMatrix\" }, \"arguments\": [ { \"type\": \"ArrayExpression\", \"elements\": [ { \"type\": \"Literal\", \"value\": 1.2 }, { \"type\": \"Literal\", \"value\": 0.5 }, { \"type\": \"Literal\", \"value\": 0.9 }, { \"type\": \"Literal\", \"value\": 1.1 } ] }, { \"type\": \"Literal\", \"value\": 2 }, { \"type\": \"Literal\", \"value\": 2 } ] } } ], \"kind\": \"let\" },\n    { \"type\": \"ExpressionStatement\", \"expression\": { \"type\": \"CallExpression\", \"callee\": { \"type\": \"Identifier\", \"name\": \"multiply\" }, \"arguments\": [ { \"type\": \"Identifier\", \"name\": \"solver\" }, { \"type\": \"Identifier\", \"name\": \"tensor\" } ] } },\n    { \"type\": \"ExpressionStatement\", \"expression\": { \"type\": \"CallExpression\", \"callee\": { \"type\": \"Identifier\", \"name\": \"drop\" }, \"arguments\": [ { \"type\": \"Identifier\", \"name\": \"solver\" } ] } }\n  ]\n}",
      'metrics': {'tokens': 142, 'bytes': 480},
      'risk': 'Low Syntax / High Logic (Deeply nested arrays cause LLM tracking failure)'
    },
    'slop': {
      'name': 'SLOP / S-Expressions',
      'code': "(def-stabilizer microgrid\n  (bind solver (alloc-wasm-solver))\n  (bind tensor (matrix 2 2 (list 1.2 0.5 0.9 1.1)))\n  (bind result (mul solver tensor))\n  (drop solver)\n  result)",
      'metrics': {'tokens': 46, 'bytes': 162},
      'risk': 'Medium (Unbalanced parentheses, undefined behavior outside core macros)'
    }
  }
}

with open('static-site/src/data/frontend_data.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2)
