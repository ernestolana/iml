# Quick Start: Your First IML Program

Welcome to IML! In this tutorial, you will write a minimal IML program using the Svelte semantic overlay, validate it, and execute it securely in the Wasm sandbox. 

We will focus on getting immediate results.

## Step 1: Launch the Svelte UI

IML is designed to be edited via a bidirectional semantic overlay powered by a Svelte frontend (`iml-translator`).

1. Open your terminal and start the local Svelte UI server:
   ```bash
   cd iml-translator/frontend
   npm install
   npm run dev
   ```
2. Open your browser to the local URL provided in the terminal (usually `http://localhost:5173`).

## Step 2: Write Your First IML Program

In the Svelte UI editor, type the following readable text block:

```text
module main:
  print "Hello, IML Sandbox!"
```

The UI automatically translates this down into IML's ultra-terse machine-state JSON in real-time. 

Click **Save to Workspace** or manually copy the generated JSON into a new file called `src/agent_logic.json`:

```json
{
  "t": "module",
  "c": [
    {
      "t": "print",
      "o": "Hello, IML Sandbox!"
    }
  ]
}
```

## Step 3: Verify the File

Before execution, you can verify that the CLI correctly parses your JSON back into the human-readable format.

Run the following command from your workspace root:

```bash
iml format --to-human src/agent_logic.json
```

*Expected output:* You should see the natural language block from Step 2 printed in your terminal.

## Step 4: Execute in the Wasm Sandbox

IML uses a secure `wasmtime` execution sandbox enhanced by the Wasm Component Model UIG. This ensures isolated execution.

Execute your validated file:

```bash
iml run --sandbox src/agent_logic.json
```

**Expected Output:**
```
Hello, IML Sandbox!
```

## Next Steps

Congratulations! You've successfully written, validated, and executed an IML program.

*   Want to perform specific tasks? Check out our [How-to Guides](../how-to/).
*   Want to dive into the architecture? Head to the [Explanation](../explanation/) section.
*   Need technical specifics? See the [Reference](../reference/).
