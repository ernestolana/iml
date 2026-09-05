
import re

with open('README.md', 'r', encoding='utf-8') as f:
    content = f.read()

new_section = """## Where to Start

If you are new to IML, jumping straight into token-dense JSON graphs can feel overwhelming. We recommend starting with an intuitive understanding of our core concepts:

- Think of IML not as a traditional language where you write text for a compiler to parse. Instead, **IML skips the text phase entirely.** 
- AI agents output the exact memory layout (the Abstract Syntax Tree) directly as a highly compressed JSON array.
- Every operation in this tree is mapped by a simple integer index. You construct programs by having operations directly refer to the index numbers of their dependencies.
- Because humans don't read raw JSON well, we use a **Semantic Overlay**?"a bidirectional UI that instantly reads the AST and explains it to you in readable, plain English steps.

To take your first steps, learn how to build your first machine-native AST node and translate it to English. 
👉 **Head over to the [Quick Start Guide](docs/tutorials/quick-start.md).**

"""

# Insert right before ## Ultra-Terse JSON Schema & Dual-State Architecture
pattern = r"(## Ultra-Terse JSON Schema & Dual-State Architecture)"
replacement = new_section + r"\1"

updated_content = re.sub(pattern, replacement, content)

with open('README.md', 'w', encoding='utf-8') as f:
    f.write(updated_content)
