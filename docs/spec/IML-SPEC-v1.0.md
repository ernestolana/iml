# IML Specification v1.0

## 1. Introduction and Scope
IML is explicitly positioned as a specialized Intermediate Representation (IR) engineered for:
- Deterministic agent workflows
- Computational graphs
- Capability-restricted host interactions

Unlike traditional programming languages or unstructured JSON outputs, IML is an AST-level, ultra-terse bytecode format. It provides mathematical guarantees around resource bounds, cyclic references, and operational safety before execution within its sandbox.

## 2. The Opcode Set (Single-Character Schema)
To maximize token density in LLM contexts, IML represents the Abstract Syntax Tree (AST) using a constrained, single-character JSON schema. The complete opcode set is as follows:

- `t` (Type): Defines the node type (e.g., operation, resource, constant).
- `c` (Children): An array of child node indices, mapping the edges of the computational graph.
- `r` (Reference/Resource): A pointer to a specific resource index or capability handle.
- `o` (Operation): The specific operation or instruction to execute at this node.
- `f` (Fuel): The maximum execution fuel or deterministic gas allocated to this subtree or node.
- `a` (Arguments/Attributes): Additional literal values, arguments, or metadata required by the operation.

## 3. Linear Type System and Mathematical Bounds
IML employs a Two-Pass Linear Type Checker to guarantee structural safety and bounded execution before runtime.

### 3.1 Acyclicity (Pass 1)
The IML IR is strictly represented as a Directed Acyclic Graph (DAG). The type checker mathematically guarantees acyclicity by traversing the `c` (Children) and `r` (Reference) indices, ensuring that no topological sorting cycle exists. Any violation results in an immediate `CycleDetected` diagnostic. This provides an absolute upper bound on graph traversal depth.

### 3.2 Resource Consumption (Pass 2)
IML enforces strict linear typing rules regarding resource lifecycles (`r`). It mathematically bounds resource consumption by ensuring that every allocated handle or complex type is consumed *exactly once*.
- A resource consumed zero times is an unconsumed leak.
- A resource consumed more than once is a potential use-after-free or cloning violation.

This ensures deterministic resource accounting and memory safety throughout the execution lifecycle.
