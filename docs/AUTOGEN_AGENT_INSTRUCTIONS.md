# IML System Instructions for AutoGen Agents

You are interacting with the Intelligent Machine Language (IML) ecosystem. IML is a specialized Intermediate Representation (IR) designed explicitly for AI agents, deterministic workflows, and capability-restricted host interactions. 

When you are asked to generate or manipulate IML code, you must strictly adhere to the following rules to ensure your output is successfully compiled and executed by the host sandbox:

## 1. Output Format (Flat Arena AST)
You must NOT output raw human-readable source code (like Python, Rust, or C). Instead, you must output a flat JSON array of nodes representing a Directed Acyclic Graph (DAG) Abstract Syntax Tree (AST). 

Use the ultra-terse single-character schema:
- `t`: Node Type / Opcode. Valid opcodes are `L` (Load), `D` (Drop), `V` (Constant Value), `T` (Tensor), `C` (Call), `B` (Branch), `P` (Print).
- `c`: Children / Input dependency indices (Array of node integers, e.g., `[0, 1]`).
- `r`: Semantic Rationale (Human-readable string comment).
- `o`: Output / Resource Handle Identifier (Optional string/symbol).
- `a`: Inline Arguments / Literals (Optional payload).

Example of a valid IML AST:
```json
{
  "nodes": [
    {"t": "L", "c": [], "r": "Allocate host solver resource", "o": "solver_handle"},
    {"t": "V", "c": [], "r": "Define parameter", "a": 42},
    {"t": "C", "c": [0, 1], "r": "Compute using solver"},
    {"t": "D", "c": [0], "r": "Drop resource to prevent memory leak"}
  ]
}
```

## 2. Acyclicity and Valid References
Your node array is evaluated strictly top-to-bottom. The `c` (Children) array must ONLY contain integers that are **strictly less than** the current node's index. Forward references, backward references causing loops, or cyclic dependencies will cause an immediate `CycleDetected` error in the Two-Pass Linear Type Checker.

## 3. Strict Resource Consumption (Linear Typing)
Any node that allocates a resource or capability handle from the host (e.g., type `L` / Alloc) must be consumed **exactly once** by a termination node (e.g., `D` / Drop).
- If you allocate a resource and fail to drop it, execution will halt statically with an `UnconsumedResource` error.
- If you drop it twice or reuse it invalidly, execution will halt with a `DoubleConsume` error.

## 4. Self-Repair Feedback
If you receive a visual diagnostic or error trace from the supervisor (e.g., `UnconsumedResource(0)`), this means node index `0` in your previous output violated linear typing rules. Adjust your JSON array by appending a drop/consumption node referencing the exact index, and resubmit.

## 5. AutoGen Integration
If you are generating IML using AutoGen, you can utilize the official `autogen-iml` SDK. It provides an `execute_iml_sandbox` tool (a `FunctionTool`), an `AssistantAgent` factory, and a `RoundRobinGroupChat` swarm template that pairs the coder with an execution agent to automatically handle Wasm sandbox feedback and self-correction.
