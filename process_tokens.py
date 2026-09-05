import json
import tiktoken
import os

with open('static-site/src/data/raw_showcase.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

enc = tiktoken.get_encoding("cl100k_base")

def get_metrics(text):
    return {
        "tokens": len(enc.encode(text)),
        "bytes": len(text.encode('utf-8'))
    }

final_data = []

# IML is special, combining AST and overlay for metrics
iml = data['iml']
combined_iml = iml['ast'] + "\n" + iml['overlay']
final_data.append({
    "id": "iml",
    "name": iml["name"],
    "code": iml["ast"],
    "overlay": iml["overlay"],
    "trace": iml["trace"],
    "tokens": get_metrics(combined_iml)["tokens"],
    "bytes": get_metrics(combined_iml)["bytes"]
})

for key in ["python", "rust", "verbose_json", "slop"]:
    item = data[key]
    final_data.append({
        "id": key,
        "name": item["name"],
        "code": item["code"],
        "tokens": get_metrics(item["code"])["tokens"],
        "bytes": get_metrics(item["code"])["bytes"]
    })

with open('static-site/src/data/showcase.json', 'w', encoding='utf-8') as f:
    json.dump(final_data, f, indent=2)

print("Generated showcase.json")
