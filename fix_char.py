
import re

with open('README.md', 'r', encoding='utf-8') as f:
    text = f.read()

text = text.replace('Semantic Overlay?"a bidirectional', 'Semantic Overlay - a bidirectional')

with open('README.md', 'w', encoding='utf-8') as f:
    f.write(text)
