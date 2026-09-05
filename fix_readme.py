
with open('README.md', 'r', encoding='utf-8') as f:
    text = f.read()

text = text.replace('Delete the `Alloc` node', 'Delete the `Drop` node')

with open('README.md', 'w', encoding='utf-8') as f:
    f.write(text)
