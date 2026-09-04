<script lang="ts">
    import { presets } from './lib/presets';
    import { validate_ast, translate_to_human, translate_from_human, simulate_execution } from './lib/wasm/iml_web';

    type PresetKey = keyof typeof presets;
    
    let selectedPreset = $state<PresetKey>('robotic_trajectory');
    
    let humanText = $state(presets.robotic_trajectory.human);
    let astJson = $state(presets.robotic_trajectory.ast);
    
    let validationResult = $state('OK');
    let simulationTrace = $state('[]');
    
    let isEditingHuman = $state(false);
    
    function loadPreset(key: PresetKey) {
        selectedPreset = key;
        humanText = presets[key].human;
        astJson = presets[key].ast;
        runValidation();
    }
    
    function runValidation() {
        validationResult = validate_ast(astJson);
        simulationTrace = simulate_execution(astJson, 100);
    }
    
    $effect(() => {
        // Trigger validation when AST changes
        runValidation();
    });
    
    function handleHumanInput(e: Event) {
        isEditingHuman = true;
        const target = e.target as HTMLTextAreaElement;
        humanText = target.value;
        try {
            astJson = translate_from_human(humanText);
            runValidation();
        } catch (err) {
            // keep old AST if invalid parse
        }
    }

    function handleAstInput(e: Event) {
        isEditingHuman = false;
        const target = e.target as HTMLTextAreaElement;
        astJson = target.value;
        try {
            if (validate_ast(astJson) === "OK") {
                humanText = translate_to_human(astJson);
            }
        } catch(err) {}
    }
    
    function downloadFile(filename: string, content: string) {
        const element = document.createElement('a');
        element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(content));
        element.setAttribute('download', filename);
        element.style.display = 'none';
        document.body.appendChild(element);
        element.click();
        document.body.removeChild(element);
    }
    
    function downloadIML() { downloadFile('workspace.iml', astJson); }
    function downloadTXT() { downloadFile('rationale.txt', humanText); }
    function downloadWAT() { downloadFile('simulation.wat', simulationTrace); }
</script>

<main class="ide-container">
    <header class="toolbar">
        <div class="preset-selector">
            <label for="preset">Load Preset: </label>
            <select id="preset" bind:value={selectedPreset} onchange={(e) => loadPreset(e.currentTarget.value as PresetKey)}>
                <option value="robotic_trajectory">Robotic Trajectory Task</option>
                <option value="quantum_matrix">Quantum State Matrix</option>
                <option value="infinite_loop">Infinite Loop Trap</option>
            </select>
        </div>
        <div class="export-tools">
            <button onclick={downloadIML}>Download .iml</button>
            <button onclick={downloadTXT}>Download .txt</button>
            <button onclick={downloadWAT}>Download .wat</button>
        </div>
    </header>

    <div class="columns">
        <!-- Column 1: Semantic English -->
        <section class="column">
            <h2>Semantic English Overlay</h2>
            <textarea 
                class="editor" 
                value={humanText} 
                oninput={handleHumanInput}
                placeholder="0: Alloc..."></textarea>
        </section>

        <!-- Column 2: Machine-Native AST -->
        <section class="column">
            <h2>Machine-Native AST Arena</h2>
            <textarea 
                class="editor ast-editor" 
                value={astJson} 
                oninput={handleAstInput}
                placeholder={"{" + " ... " + "}"}></textarea>
        </section>

        <!-- Column 3: Diagnostics & Runtime -->
        <section class="column diagnostics">
            <h2>Diagnostics & Runtime</h2>
            <div class="panel">
                <h3>Validation Telemetry</h3>
                <pre class="telemetry {validationResult === 'OK' ? 'ok' : 'error'}">
                    {validationResult}
                </pre>
            </div>
            <div class="panel">
                <h3>Execution Engine (Fuel: 100)</h3>
                <pre class="simulation">{simulationTrace}</pre>
            </div>
        </section>
    </div>
</main>

<style>
    :global(body) {
        margin: 0;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
        background-color: #1e1e1e;
        color: #d4d4d4;
    }

    .ide-container {
        display: flex;
        flex-direction: column;
        height: 100vh;
        overflow: hidden;
    }

    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 10px 20px;
        background-color: #2d2d2d;
        border-bottom: 1px solid #3c3c3c;
    }

    .toolbar select, .toolbar button {
        background-color: #3c3c3c;
        color: #d4d4d4;
        border: 1px solid #555;
        padding: 5px 10px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
        margin-left: 10px;
    }

    .toolbar select:hover, .toolbar button:hover {
        background-color: #4a4a4a;
    }

    .columns {
        display: flex;
        flex: 1;
        overflow: hidden;
    }

    .column {
        flex: 1;
        display: flex;
        flex-direction: column;
        border-right: 1px solid #3c3c3c;
        padding: 10px;
    }

    .column:last-child {
        border-right: none;
    }

    h2 {
        font-size: 14px;
        text-transform: uppercase;
        color: #9cdcfe;
        margin-bottom: 10px;
        margin-top: 0;
    }

    .editor {
        flex: 1;
        background-color: #1e1e1e;
        color: #ce9178;
        border: 1px solid #3c3c3c;
        font-family: "Consolas", "Courier New", monospace;
        padding: 10px;
        resize: none;
        outline: none;
        white-space: pre;
        overflow: auto;
    }

    .ast-editor {
        color: #b5cea8;
    }

    .editor:focus {
        border-color: #007acc;
    }

    .diagnostics {
        background-color: #252526;
    }

    .panel {
        flex: 1;
        display: flex;
        flex-direction: column;
        margin-bottom: 10px;
    }

    .panel:last-child {
        margin-bottom: 0;
    }

    h3 {
        font-size: 12px;
        color: #c586c0;
        margin: 0 0 5px 0;
    }

    pre {
        flex: 1;
        margin: 0;
        background-color: #1e1e1e;
        border: 1px solid #3c3c3c;
        padding: 10px;
        overflow: auto;
        font-family: "Consolas", "Courier New", monospace;
        font-size: 12px;
    }

    .ok {
        color: #4CAF50;
    }

    .error {
        color: #F44336;
    }
</style>
