# IML Specification v0.2 Draft

## 1. Introduction and Scope
IML is explicitly positioned as a specialized Intermediate Representation (IR) engineered for:
- Deterministic agent workflows
- Computational graphs
- Capability-restricted host interactions

Unlike traditional programming languages or unstructured JSON outputs, IML is an AST-level, ultra-terse bytecode format. It provides mathematical guarantees around resource bounds, cyclic references, and operational safety before execution within its sandbox.

## 2. Flat Arena AST Structure
The IML AST is strictly represented as a flat arena. The required root structure is:
```json
{
  "nodes": [ ... ]
}
```
All operations, values, and logic must be defined as nodes within this flat array.

## 3. Standardized Node Schema
Each node in the array follows a strict, standardized schema designed to maximize token density:

- `t` (Node Type / Opcode): Tagged object or string representing the operation.
- `c` (Children): Array of input dependency indices (integer offsets pointing to prior nodes, e.g., `[0, 1]`).
- `r` (Semantic Rationale): Human-readable string comment detailing the node's intent.
- `o` (Output / Resource Handle Identifier): Optional string/symbol identifying an allocated resource.
- `a` (Inline Arguments / Literals): Optional payload or constant data.

## 4. Core Opcode Set (v0.2)
To ensure sandbox integrity and deterministic evaluation, IML defines the following core opcodes. Note that operations like File I/O and HTTP are *not* raw opcodes. They must be loaded as external WASI 0.2 capability components via `L` and invoked via `C`.

- `L`: Load / Allocate External Resource/Wasm Component Handle.
- `D`: Drop / Free Resource Handle (mandatory linear release).
- `V`: Constant Value / Literal payload provided in `a`.
- `T`: Tensor / State Array / Matrix declaration.
- `C`: Call / Compute (invokes an allocated Wasm function or host intrinsic).
- `B`: Branch (conditional evaluation between two node indices).
- `P`: Print / Host Log emit via standard debug pipe.

## 5. Linear Type System and Mathematical Bounds
IML employs a Two-Pass Linear Type Checker to guarantee structural safety and bounded execution before runtime.

### 5.1 Acyclicity (Pass 1)
The IML IR is strictly represented as a Directed Acyclic Graph (DAG). The type checker guarantees acyclicity by traversing the `c` (Children) array. Node references must strictly point to indices *prior* to the current node. Any forward or self-reference yields an immediate `CycleDetected` diagnostic.

### 5.2 Resource Consumption (Pass 2)
IML enforces strict linear typing rules regarding resource lifecycles. It bounds resource consumption by ensuring that every allocated handle (`L`) is consumed exactly once by a termination node (`D`).
- A resource consumed zero times is an unconsumed leak.
- A resource consumed more than once is a potential use-after-free or cloning violation.
