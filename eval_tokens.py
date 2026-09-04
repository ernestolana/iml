
import json

def estimate_tokens(text: str) -> int:
    try:
        import tiktoken
        enc = tiktoken.get_encoding("cl100k_base")
        return len(enc.encode(text))
    except ImportError:
        # Fallback estimation: 1 token ~ 4 characters for code
        return max(1, len(text) // 4)

task_a_iml = {"nodes": [{"t":{"N":0}}, {"t":{"N":1}}, {"t":"A","c":[0,1]}]}
task_a_verbose = {
    "type": "Block",
    "statements": [
        {"type": "VariableDeclaration", "name": "a", "value": {"type": "Literal", "value": 0}},
        {"type": "VariableDeclaration", "name": "b", "value": {"type": "Literal", "value": 1}},
        {"type": "BinaryExpression", "operator": "+", "left": "a", "right": "b"}
    ]
}

iml_str = json.dumps(task_a_iml, separators=(",",":"))
verb_str = json.dumps(task_a_verbose)

print("IML Token Estimate:", estimate_tokens(iml_str))
print("Verbose JSON Token Estimate:", estimate_tokens(verb_str))

