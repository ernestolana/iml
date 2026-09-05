# Intelligent Machine Language (IML)

## Illustrative Architecture & Comparisons

The [Live Showcase](https://iml-interface.web.app) provides a comprehensive breakdown of the dual-state architecture and visual comparisons between IML and traditional syntax formats.

### Core Benchmark: Microgrid Power Stabilization
| Language / Format | Byte Size | Tokens (cl100k) | AI Generation Cost |
| :--- | :--- | :--- | :--- |
| **IML (Machine AST)** | **198 B** | **59** | **Lowest** |
| Verbose JSON AST | 1,029 B | 275 | ~4.6x more |
| Rust | 158 B | 49 | Fast, but unconstrained |
| Python 3.12 | 134 B | 36 | Fast, but hallucinates |
| SLOP / S-Expressions | 129 B | 45 | Too ambiguous |

*Note: IML strikes the perfect balance by retaining the strict deterministic qualities of a raw syntax tree, while heavily compressing the keys and structures to respect the AI token economy.*