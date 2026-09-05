# Intelligent Machine Language (IML)

## Illustrative Architecture & Comparisons

The [Live Architectural Showcase](https://iml-interface.web.app) provides a comprehensive, interactive breakdown of the entire IML architecture:

- **Multi-Language Token Matrix**: Interactive tabs comparing IML tokens directly against Python, Rust, and standard JSON.
- **Dual-State Engine**: Live mapping between Semantic English rationales and Machine-Native AST bytecode.
- **Interactive ROI Calculator**: Calculate exact dollar savings when scaling agent deployments to millions of calls.
- **Sandboxed Execution Toggles**: Visual simulator demonstrating IML's response to infinite loops and unconsumed memory leaks.

Explore the live environment at: [https://iml-interface.web.app](https://iml-interface.web.app)

### Core Benchmark: Microgrid Power Stabilization
| Language / Format | Byte Size | Tokens (cl100k) | AI Generation Cost |
| :--- | :--- | :--- | :--- |
| **IML (Machine AST)** | **118 B** | **15** | **Lowest** |
| SLOP / S-Expressions | 162 B | 46 | Too ambiguous |
| Python 3.12 | 254 B | 78 | Fast, but hallucinates |
| Rust | 310 B | 94 | Fast, but unconstrained |
| Verbose JSON AST | 480 B | 142 | ~9.5x more |

*Note: IML strikes the perfect balance by retaining the strict deterministic qualities of a raw syntax tree, while heavily compressing the keys and structures to respect the AI token economy.*