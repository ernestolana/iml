
with open('README.md', 'a', encoding='utf-8') as f:
    f.write('''
## 🎮 Live Interactive Playground

[![Live Interactive Playground](https://img.shields.io/badge/Live-Interactive_Playground-success?style=for-the-badge)](https://iml-interface.web.app)

Experience the dual-state architecture of IML firsthand in your browser via our [**Live Interactive Playground**](https://iml-interface.web.app). The playground runs our Rust-based `iml-web` Wasm bridge entirely client-side, giving you real-time feedback with zero backend dependencies.

### How to Use It
The playground is split into a 3-column IDE layout:
1. **Semantic English Overlay (Left)**: This is the human-readable representation of the AST. It translates the raw index-based graph into sequential operations.
2. **Machine-Native AST Arena (Center)**: This is the raw JSON schema utilizing ultra-terse keys (`t`, `c`, `r`, `o`). This is exactly what an LLM agent outputs.
3. **Diagnostics & Runtime (Right)**: Here you'll see the live output from our strict Two-Pass Linear Type Checker and the Sandboxed Execution Engine. 

When you type in the AST Arena or the Semantic Overlay, the playground instantly cross-compiles and validates your changes on every keystroke. 

### Things to Try
Load the pre-configured presets using the dropdown toolbar, and try these modifications:

* **Robotic Trajectory Task**
  * *Try this*: Delete the `Alloc` node in the Semantic English overlay.
  * *What to expect*: The Machine-Native AST will instantly update, and the Diagnostics panel will throw a `RepairError` trace (e.g. an `UnconsumedResource` or a missing reference), mimicking the exact constrained feedback loop an LLM receives.
* **Quantum State Matrix**
  * *Try this*: In the Machine-Native AST Arena, duplicate the index `2` reference in the `Drop` node's children (`"c": [2, 2]`).
  * *What to expect*: The linear type checker will trap it and throw a `DoubleConsume(2)` error, guaranteeing memory safety for resources.
* **Infinite Loop Trap**
  * *Try this*: Select this preset to run code that points to itself (`0: Add -> 0`).
  * *What to expect*: The execution engine immediately traps the execution via deterministic fuel exhaustion (`Trap: Fuel exhaustion`) rather than hanging your browser, demonstrating how agentic hallucination loops are safely mitigated.
''')
