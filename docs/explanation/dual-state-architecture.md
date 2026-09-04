# Explanation: Dual-State Architecture & Neuro-Symbolic Integration

IML replaces traditional parsing pipelines with a **Dual-State Architecture**. This design separates the code's computational reality (Machine State) from the human-readable intent (Semantic Overlay). 

## The Flat Arena (Machine State)

Traditional human languages use deeply nested syntax trees (ASTs). When fed into LLMs, deeply nested JSON or S-expressions quickly exhaust context windows and increase the probability of hallucinated brackets or cyclic structures. 

To solve this, IML uses a flat **Arena** (`Vec<Node>`). Every node is stored linearly and identified by an integer index. Relationships are represented as arrays of integers (e.g., `children: [1, 2]`) instead of nested objects. 
Coupled with strictly enforced single-character keys (`"t"` for type, `"c"` for children), the machine state dramatically reduces token footprint. This ultra-terse representation empowers agents to generate massive logical structures natively without syntax overhead.

## The Semantic Overlay and Neuro-Symbolic Hook

If the Machine State is meant strictly for LLM processing and execution, how do human engineers debug it? 

IML introduces the **Semantic Overlay**, a bidirectional Svelte-powered frontend (`iml-translator`). It reconstructs the flat Arena back into a readable, tree-like format (`to_human_readable`).

Crucially, changes made in the Semantic Overlay are not "parsed" back through traditional lexical analysis. Instead, the architecture utilizes a **Neuro-Symbolic Hook**. When a human alters the semantic text, a `TranslateRequest` triggers the backend `rewrite_node` function. This leverages the LLM to cleanly transition the human's fuzzy semantic intent back into strict, valid Arena JSON. By treating the LLM as the compiler's frontend, IML bridges the gap between human readability and machine token-density.
