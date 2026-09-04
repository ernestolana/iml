# Explanation: Linear Memory Ownership and Wasm Constraints

One of the most persistent issues with LLM-generated code is resource mismanagement. Because autoregressive models generate code token-by-token, they often instantiate objects or allocate memory but "forget" to close or free them, leading to catastrophic leaks at runtime.

IML intercepts this behavior statically via the **Two-Pass Linear Type Checker**, ensuring memory safety before execution ever reaches the WebAssembly sandbox.

## Why Wasm Needs Explicit Memory Management

IML code executes within the Wasmtime Component Model. WebAssembly environments are heavily sandboxed and operate under strict deterministic constraints:
- They use a linear memory model without automatic runtime garbage collection for external component handles.
- The `SandboxLimiter` enforces strict `max_memory_bytes` and `fuel_limit` caps.

If an LLM allocates a host resource—such as opening a file handle or generating a large matrix—and fails to free it, the sandbox will rapidly exhaust its memory quota and trap.

## The Two-Pass Linear Checker

Rather than catching these leaks at runtime (which wastes compute and execution fuel), IML analyzes the flat AST beforehand.

1. **Pass 1 (Bounds & Cycle Detection)**: Validates that all node pointers point to valid indices and that the graph has no cycles. This guarantees the AST is finite and analyzable.
2. **Pass 2 (Resource Lifecycle)**: The checker identifies every single `NodeType::Alloc` (creation of a resource). It then scans the graph for `NodeType::Drop` (explicit memory drops). Under linear typing rules, **every allocation must be consumed exactly once**. 
   - If an allocation is never passed to a Drop node, it returns an `UnconsumedResource` error.
   - If it is passed multiple times, it returns a `DoubleConsume` error.

By forcing the LLM to represent resource destruction as an explicit logical node (`"D"`) connected to the creation node, IML forces the model to mathematically account for its memory footprint. If the agent hallucinates a leak, the linear type checker immediately returns a cheap, deterministic error trace (e.g., `UnconsumedResource(4)`), allowing the agent to self-repair in a fast neuro-symbolic feedback loop.
