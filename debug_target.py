import re

with open('static-site/src/main.js', 'r', encoding='utf-8') as f:
    text = f.read()

for line in text.split('\n'):
    if 'data-target' in line or 'id="pane-' in line:
        print(line.strip())
