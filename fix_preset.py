
with open('playground/src/lib/presets.ts', 'r', encoding='utf-8') as f:
    text = f.read()

old_human = '"0: Alloc\\n1: Var(robot_arm)\\n2: Add -> 0, 1\\n"'
new_human = '"0: Alloc\\n1: Var(robot_arm)\\n2: Add -> 0, 1\\n3: Drop -> 0\\n"'

old_ast = """        ast: JSON.stringify({
            nodes: [
                { t: { L: null }, c: [], r: "Allocate memory for trajectory", o: "mem1" },
                { t: { V: "robot_arm" }, c: [], r: "Robot arm state" },
                { t: { A: null }, c: [0, 1], r: "Combine allocation and state" }
            ]
        }, null, 2)"""

new_ast = """        ast: JSON.stringify({
            nodes: [
                { t: { L: null }, c: [], r: "Allocate memory for trajectory", o: "mem1" },
                { t: { V: "robot_arm" }, c: [], r: "Robot arm state" },
                { t: { A: null }, c: [0, 1], r: "Combine allocation and state" },
                { t: { D: null }, c: [0], r: "Free memory" }
            ]
        }, null, 2)"""

text = text.replace(old_human, new_human)
text = text.replace(old_ast, new_ast)

with open('playground/src/lib/presets.ts', 'w', encoding='utf-8') as f:
    f.write(text)
