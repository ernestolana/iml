import data from './data/frontend_data.json';

function init() {
  const app = document.getElementById('app');
  
  const iml = data.iml;
  const python = data.comparisons.python;
  const rust = data.comparisons.rust;
  const verbose = data.comparisons.verbose_json;
  const slop = data.comparisons.slop;

  const competitors = [python, rust, verbose, slop];

  // Pre-formatted AST as requested
  const imlAstRendered = iml.ast;

  const html = `
    <!-- Section 1: Hero -->
    <header class="py-24 px-4 relative overflow-hidden" aria-labelledby="hero-title">
      <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-cyan-900/20 via-slate-950 to-slate-950 pointer-events-none"></div>
      <div class="max-w-5xl mx-auto text-center relative z-10">
        <h1 id="hero-title" class="text-5xl md:text-7xl font-extrabold tracking-tight text-white mb-6">
          Code for <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-500">Machines</span>,<br /> Not Humans.
        </h1>
        <p class="text-xl text-slate-400 mb-10 max-w-3xl mx-auto leading-relaxed">
          <strong>For Founders:</strong> Cut LLM API costs by up to 80% with hyper-dense bytecode generation.<br/>
          <strong>For Engineers:</strong> Eliminate syntax hallucinations and memory leaks via strict, deterministic Sandboxed ASTs.
        </p>
        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <a href="#matrix" class="inline-flex items-center justify-center gap-2 bg-cyan-500 hover:bg-cyan-400 text-slate-950 px-8 py-4 rounded-full font-bold transition-all shadow-[0_0_20px_rgba(6,182,212,0.3)] hover:shadow-[0_0_30px_rgba(6,182,212,0.5)] focus:outline-none focus:ring-4 focus:ring-cyan-500/50">
            View Token Matrix
          </a>
          <a href="#roi-calculator" class="inline-flex items-center justify-center gap-2 bg-slate-800 hover:bg-slate-700 text-white px-8 py-4 rounded-full font-bold transition-all border border-slate-700 focus:outline-none focus:ring-4 focus:ring-slate-500/50">
            Calculate ROI
          </a>
        </div>
      </div>
    </header>

    <!-- Section 2: Multi-Language Token Matrix -->
    <section id="matrix" class="py-24 px-4 border-t border-slate-900 bg-slate-950/50" aria-labelledby="matrix-title">
      <div class="max-w-7xl mx-auto">
        <div class="text-center mb-16">
          <h2 id="matrix-title" class="text-3xl md:text-4xl font-bold text-white mb-4">The Multi-Language Token Matrix</h2>
          <p class="text-slate-400 max-w-2xl mx-auto">See exactly how IML destroys the LLM context window bottleneck on a standard "Microgrid Power Stabilization" task.</p>
        </div>

        <div class="grid lg:grid-cols-2 gap-8 items-start">
          <!-- Competitor Viewer -->
          <div class="bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-2xl flex flex-col h-[550px]">
            <div class="flex overflow-x-auto border-b border-slate-800 bg-slate-950 no-scrollbar" role="tablist" aria-label="Language Comparison Tabs">
              ${competitors.map((c, i) => `
                <button class="tab-btn px-6 py-4 text-sm font-semibold transition-all whitespace-nowrap focus:outline-none focus:ring-2 focus:ring-inset focus:ring-cyan-500 ${i === 0 ? 'text-white border-b-2 border-cyan-500 bg-slate-900' : 'text-slate-500 hover:text-slate-300 hover:bg-slate-900/50'}" data-target="${c.id}" role="tab" aria-selected="${i === 0 ? 'true' : 'false'}" aria-controls="pane-${c.id}">
                  ${c.name}
                </button>
              `).join('')}
            </div>
            
            <div class="relative flex-1 overflow-hidden flex flex-col bg-[#1d1f21]" id="tab-contents">
              ${competitors.map((c, i) => `
                <div class="tab-pane absolute inset-0 flex flex-col ${i === 0 ? 'block' : 'hidden'}" id="pane-${c.id}" role="tabpanel" aria-labelledby="tab-${c.id}">
                  <!-- Dynamic Metrics Pill -->
                  <div class="bg-slate-950/80 backdrop-blur border-b border-slate-800 p-4 grid grid-cols-2 gap-4 text-xs font-mono">
                    <div><span class="text-slate-500 block mb-1">Size</span><span class="text-rose-400 font-bold">${c.metrics.tokens} Tokens</span> (${c.metrics.bytes} B)</div>
                    <div><span class="text-slate-500 block mb-1">Hallucination Risk</span><span class="text-amber-400">${c.risk}</span></div>
                  </div>
                  <div class="flex-1 overflow-auto p-6">
                    <pre><code class="language-${c.id === 'python' ? 'python' : c.id === 'rust' ? 'rust' : c.id === 'slop' ? 'lisp' : 'json'}">${c.code}</code></pre>
                  </div>
                </div>
              `).join('')}
            </div>
          </div>

          <!-- IML Winner -->
          <div class="bg-slate-950 border border-cyan-500/30 rounded-2xl overflow-hidden shadow-[0_0_50px_rgba(6,182,212,0.15)] flex flex-col relative h-[550px] ring-1 ring-cyan-500/20">
            <div class="absolute top-0 right-0 bg-gradient-to-r from-cyan-500 to-blue-600 text-white text-xs font-bold px-4 py-1.5 rounded-bl-xl shadow-lg z-10 uppercase tracking-widest">Target Architecture</div>
            <div class="flex border-b border-cyan-900/50 bg-slate-900 p-4">
              <h3 class="font-bold text-cyan-400 text-lg">IML Flat Arena (Machine AST)</h3>
            </div>
            <div class="flex flex-col flex-1 bg-[#1d1f21] relative overflow-hidden">
              <div class="bg-slate-950/80 backdrop-blur border-b border-slate-800 p-4 grid grid-cols-2 gap-4 text-xs font-mono">
                <div><span class="text-slate-500 block mb-1">Size</span><span class="text-cyan-400 font-bold text-base">15 Tokens</span> (118 B)</div>
                <div><span class="text-slate-500 block mb-1">Hallucination Risk</span><span class="text-green-400 font-bold">Zero (Deterministic Sandboxing)</span></div>
              </div>
              <div class="flex-1 overflow-auto p-6 relative">
                <pre><code class="language-json" id="iml-json-code">${imlAstRendered}</code></pre>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 3: Dual-State Architecture -->
    <section id="dual-state" class="py-24 px-4 bg-slate-900 border-t border-slate-800" aria-labelledby="dual-state-title">
      <div class="max-w-7xl mx-auto">
        <div class="text-center mb-16">
          <h2 id="dual-state-title" class="text-3xl md:text-4xl font-bold text-white mb-4">Dual-State Architecture</h2>
          <p class="text-slate-400 max-w-2xl mx-auto">IML bridges the gap between machine efficiency and human debuggability. Hover over the English steps below to see them map perfectly to the underlying JSON Flat Arena.</p>
        </div>

        <div class="grid lg:grid-cols-2 gap-8 items-stretch">
          <!-- Semantic English Overlay -->
          <div class="bg-slate-950 border border-slate-800 rounded-2xl p-6 shadow-xl flex flex-col">
            <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-6 flex items-center gap-2">
              <span class="w-2 h-2 rounded-full bg-amber-400"></span>
              Semantic English Overlay (Human)
            </h4>
            <div class="space-y-3 flex-1">
              ${iml.overlay.map((step, idx) => `
                <div class="semantic-step p-4 rounded-xl border border-slate-800 bg-slate-900 text-slate-300 text-sm cursor-pointer transition-all hover:bg-slate-800 hover:border-cyan-500 hover:text-white" data-node-index="${idx}" tabindex="0" role="button" aria-label="Highlight Node ${idx}">
                  ${step}
                </div>
              `).join('')}
            </div>
          </div>

          <!-- Correlated JSON Code -->
          <div class="bg-slate-950 border border-slate-800 rounded-2xl p-6 shadow-xl flex flex-col">
            <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-6 flex items-center gap-2">
              <span class="w-2 h-2 rounded-full bg-cyan-400"></span>
              Machine-Native AST
            </h4>
            <div class="flex-1 bg-[#1d1f21] rounded-xl overflow-hidden relative border border-slate-800">
               <pre class="h-full w-full overflow-auto p-4 m-0"><code class="language-json" id="hover-sync-json">${imlAstRendered}</code></pre>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 4: Neuro-Symbolic & Sandboxing Engine -->
    <section id="sandbox" class="py-24 px-4 bg-slate-950 border-t border-slate-800" aria-labelledby="sandbox-title">
      <div class="max-w-7xl mx-auto">
        <div class="text-center mb-16">
          <h2 id="sandbox-title" class="text-3xl md:text-4xl font-bold text-white mb-4">Neuro-Symbolic Sandboxing</h2>
          <p class="text-slate-400 max-w-2xl mx-auto">Visualizing the deterministic pipeline. IML runs inside a Wasmtime fuel-metered sandbox, guaranteeing halting and safety.</p>
        </div>

        <div class="grid lg:grid-cols-5 gap-8 items-center">
          <div class="lg:col-span-3 bg-slate-900 border border-slate-800 p-8 rounded-2xl">
            <!-- Interactive SVG Pipeline -->
            <div class="flex flex-col gap-6" role="group" aria-label="Pipeline Architecture Visualization">
              <!-- Header Toggles -->
              <div class="flex justify-center gap-4 mb-4">
                 <button id="toggle-success" class="px-4 py-2 text-sm font-bold rounded bg-green-900/50 text-green-400 border border-green-700 focus:outline-none focus:ring-2 focus:ring-green-400 transition-colors">Success Path</button>
                 <button id="toggle-leak" class="px-4 py-2 text-sm font-bold rounded bg-slate-800 text-slate-400 border border-slate-700 hover:bg-rose-900/30 hover:text-rose-400 focus:outline-none focus:ring-2 focus:ring-rose-400 transition-colors">Simulate Memory Leak</button>
                 <button id="toggle-loop" class="px-4 py-2 text-sm font-bold rounded bg-slate-800 text-slate-400 border border-slate-700 hover:bg-amber-900/30 hover:text-amber-400 focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors">Simulate Infinite Loop</button>
              </div>
              
              <!-- Diagram -->
              <svg viewBox="0 0 800 300" class="w-full h-auto drop-shadow-xl font-mono text-xs">
                <!-- Grid Background -->
                <defs>
                  <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
                    <path d="M 40 0 L 0 0 0 40" fill="none" stroke="rgba(255,255,255,0.03)" stroke-width="1"/>
                  </pattern>
                </defs>
                <rect width="100%" height="100%" fill="url(#grid)" />

                <!-- Nodes -->
                <g transform="translate(50, 130)">
                   <rect width="120" height="40" rx="6" fill="#0f172a" stroke="#0ea5e9" stroke-width="2" />
                   <text x="60" y="24" fill="#cbd5e1" text-anchor="middle">Agent Output</text>
                </g>
                <g transform="translate(250, 130)">
                   <rect width="120" height="40" rx="6" fill="#0f172a" stroke="#8b5cf6" stroke-width="2" />
                   <text x="60" y="24" fill="#cbd5e1" text-anchor="middle">Linear Checker</text>
                </g>
                <g transform="translate(450, 130)">
                   <rect width="120" height="40" rx="6" fill="#0f172a" stroke="#f59e0b" stroke-width="2" />
                   <text x="60" y="24" fill="#cbd5e1" text-anchor="middle">Wasm Sandbox</text>
                </g>
                <g transform="translate(650, 130)">
                   <rect width="120" height="40" rx="6" fill="#0f172a" stroke="#10b981" stroke-width="2" id="out-box" />
                   <text x="60" y="24" fill="#cbd5e1" text-anchor="middle" id="out-text">Output</text>
                </g>

                <!-- Connecting Lines -->
                <path d="M 170 150 L 240 150" stroke="#475569" stroke-width="2" marker-end="url(#arrow)" />
                <path d="M 370 150 L 440 150" stroke="#475569" stroke-width="2" marker-end="url(#arrow)" id="line-check" />
                <path d="M 570 150 L 640 150" stroke="#475569" stroke-width="2" marker-end="url(#arrow)" id="line-run" />

                <defs>
                  <marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#475569" />
                  </marker>
                  <marker id="arrow-red" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#ef4444" />
                  </marker>
                </defs>
                
                <!-- Feedback Error Loops -->
                <path d="M 310 130 C 310 50, 110 50, 110 130" fill="none" stroke="#ef4444" stroke-width="2" stroke-dasharray="4" marker-end="url(#arrow-red)" opacity="0" id="path-leak" />
                <text x="210" y="70" fill="#ef4444" text-anchor="middle" opacity="0" id="text-leak">UnconsumedResource(0)</text>

                <path d="M 510 130 C 510 20, 110 20, 110 130" fill="none" stroke="#ef4444" stroke-width="2" stroke-dasharray="4" marker-end="url(#arrow-red)" opacity="0" id="path-loop" />
                <text x="310" y="40" fill="#ef4444" text-anchor="middle" opacity="0" id="text-loop">Trap: Fuel Exhaustion</text>
              </svg>
            </div>
          </div>
          <div class="lg:col-span-2 bg-slate-900 border border-slate-800 rounded-2xl p-6 h-full flex flex-col">
            <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-4">Sandbox Output Trace</h4>
            <div class="bg-black/60 rounded-xl font-mono text-xs shadow-inner flex-1 p-4 overflow-auto border border-slate-800">
              <div id="sandbox-trace-output" class="text-green-400 whitespace-pre-wrap">${iml.trace}</div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 5: ROI Calculator & Benchmarks -->
    <section id="roi-calculator" class="py-24 px-4 bg-slate-900 border-t border-slate-800" aria-labelledby="roi-title">
      <div class="max-w-7xl mx-auto">
        <div class="text-center mb-16">
          <h2 id="roi-title" class="text-3xl md:text-4xl font-bold text-white mb-4">ROI & Benchmark Calculator</h2>
          <p class="text-slate-400 max-w-2xl mx-auto">See the massive financial impact of moving from Python ASTs to IML Flat Arenas at scale. (Costs estimated per 1M tokens at launch).</p>
        </div>

        <div class="grid lg:grid-cols-2 gap-12">
          <!-- Inputs -->
          <div class="bg-slate-950 p-8 rounded-2xl border border-slate-800 shadow-xl">
            <h3 class="text-xl font-bold text-white mb-6">Simulation Parameters</h3>
            
            <div class="space-y-8">
              <div>
                <label for="calc-model" class="block text-sm font-medium text-slate-400 mb-2">Language Model Pricing</label>
                <select id="calc-model" class="w-full bg-slate-900 border border-slate-700 rounded-lg p-3 text-white focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500">
                  <option value="gemini">Gemini 2.0 Flash ($0.10 in / $0.40 out)</option>
                  <option value="claude">Claude 3.5 Sonnet ($3.00 in / $15.00 out)</option>
                  <option value="gpt4o">GPT-4o ($2.50 in / $10.00 out)</option>
                </select>
              </div>

              <div>
                <div class="flex justify-between mb-2">
                  <label for="calc-calls" class="text-sm font-medium text-slate-400">Agent Calls per Day</label>
                  <span id="val-calls" class="text-cyan-400 font-mono font-bold">10,000</span>
                </div>
                <input type="range" id="calc-calls" min="100" max="1000000" step="100" value="10000" class="w-full accent-cyan-500 bg-slate-800 rounded-lg appearance-none h-2 cursor-pointer" aria-valuemin="100" aria-valuemax="1000000" aria-valuenow="10000">
              </div>

              <div>
                <div class="flex justify-between mb-2">
                  <label for="calc-steps" class="text-sm font-medium text-slate-400">Average AST Nodes / Output</label>
                  <span id="val-steps" class="text-cyan-400 font-mono font-bold">20</span>
                </div>
                <input type="range" id="calc-steps" min="5" max="500" step="5" value="20" class="w-full accent-cyan-500 bg-slate-800 rounded-lg appearance-none h-2 cursor-pointer" aria-valuemin="5" aria-valuemax="500" aria-valuenow="20">
              </div>
            </div>
          </div>

          <!-- Output Metrics -->
          <div class="flex flex-col gap-6">
            <div class="bg-gradient-to-br from-cyan-900/40 to-blue-900/20 p-8 rounded-2xl border border-cyan-800/50 shadow-xl flex-1 flex flex-col justify-center">
              <h4 class="text-sm font-bold text-cyan-400 uppercase tracking-wider mb-2">Estimated Monthly Savings</h4>
              <div class="text-6xl font-extrabold text-white mb-2" id="out-savings">$0.00</div>
              <p class="text-slate-400 text-sm">By switching from Python code generation to IML bytecode.</p>
            </div>
            
            <div class="grid grid-cols-2 gap-6">
               <div class="bg-slate-950 p-6 rounded-2xl border border-slate-800 shadow-lg text-center">
                 <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-2">Python Cost/Mo</h4>
                 <div class="text-2xl font-mono text-rose-400" id="out-python-cost">$0</div>
               </div>
               <div class="bg-slate-950 p-6 rounded-2xl border border-slate-800 shadow-lg text-center">
                 <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-2">IML Cost/Mo</h4>
                 <div class="text-2xl font-mono text-cyan-400" id="out-iml-cost">$0</div>
               </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 6: Specification -->
    <section class="py-24 px-4 bg-slate-950 border-t border-slate-900">
      <div class="max-w-4xl mx-auto text-center">
        <h2 class="text-2xl font-bold text-white mb-6">Deep Dive for Compiler Engineers</h2>
        <p class="text-slate-400 mb-8">
          IML is entirely open-source and built in Rust. Explore the grammar schemas, WASM component bridges, and linear validation types in the official repository.
        </p>
        <a href="https://github.com/ernestolana/iml" target="_blank" rel="noopener noreferrer" class="inline-flex items-center gap-2 bg-slate-800 hover:bg-slate-700 text-white px-6 py-3 rounded-full font-medium transition-colors border border-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-500">
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"></path></svg>
          Read the Specifications
        </a>
      </div>
    </section>
  `;

  app.innerHTML = html;

  setupTabs();
  setupHoverSync();
  setupSVGSimulator(iml.trace);
  setupCalculator();

  if (window.Prism) {
    window.Prism.highlightAll();
  }
}

function setupTabs() {
  const headers = document.querySelectorAll('.tab-btn');
  const panes = document.querySelectorAll('.tab-pane');

  headers.forEach(btn => {
    btn.addEventListener('click', () => {
      headers.forEach(h => {
        h.classList.remove('text-white', 'border-b-2', 'border-cyan-500', 'bg-slate-900');
        h.classList.add('text-slate-500');
        h.setAttribute('aria-selected', 'false');
      });
      panes.forEach(p => p.classList.add('hidden'));

      btn.classList.add('text-white', 'border-b-2', 'border-cyan-500', 'bg-slate-900');
      btn.classList.remove('text-slate-500');
      btn.setAttribute('aria-selected', 'true');
      
      const target = btn.getAttribute('data-target');
      document.getElementById('pane-' + target).classList.remove('hidden');
    });
  });
}

function setupHoverSync() {
  // Sync logic for Hover & Touch
  const steps = document.querySelectorAll('.semantic-step');
  
  // Wait for Prism to finish formatting
  setTimeout(() => {
    const jsonContainer = document.querySelector('#hover-sync-json');
    if (!jsonContainer) return;
    
    // We break the rendered innerHTML down line by line or object by object.
    // Since it's a 4-item array in `nodes`, we can find '{' boundaries to wrap.
    let htmlContent = jsonContainer.innerHTML;
    
    // Naive split on `{` inside the array.
    // 0: {"t":{"L":null}, ...}
    // 1: {"t":{"B":...}
    // 2: {"t":{"M":null}, ...}
    // 3: {"t":{"D":null}, ...}
    
    // To reliably wrap them post-prism, we replace them by matching the "r" rationale strings that correlate to nodes.
    const rationales = [
       "Allocate thermodynamic power grid",
       "Initialize 2x2 grid state",
       "Execute matrix stabilization",
       "Explicitly drop thermodynamic solver"
    ];
    
    rationales.forEach((r, idx) => {
       // Look for the block `{ ... r ... }` and wrap it
       const regex = new RegExp(`([ \\t]*{<br>[\\s\\S]*?${r}[\\s\\S]*?}[ \\t]*)`, 'g');
       htmlContent = htmlContent.replace(regex, `<span class="json-node-wrap transition-all duration-300 rounded" data-json-idx="${idx}">$1</span>`);
    });
    
    jsonContainer.innerHTML = htmlContent;

    const wraps = document.querySelectorAll('.json-node-wrap');

    const handleEnter = (idx) => {
      wraps.forEach(w => w.classList.remove('bg-cyan-900/50', 'ring-1', 'ring-cyan-500'));
      steps.forEach(s => s.classList.remove('bg-cyan-900/50', 'border-cyan-500'));
      
      const targetWrap = document.querySelector(`.json-node-wrap[data-json-idx="${idx}"]`);
      if (targetWrap) targetWrap.classList.add('bg-cyan-900/50', 'ring-1', 'ring-cyan-500');
      steps[idx].classList.add('bg-cyan-900/50', 'border-cyan-500');
    };

    const handleLeave = () => {
      wraps.forEach(w => w.classList.remove('bg-cyan-900/50', 'ring-1', 'ring-cyan-500'));
      steps.forEach(s => s.classList.remove('bg-cyan-900/50', 'border-cyan-500'));
    };

    steps.forEach((step, idx) => {
      // Mouse events
      step.addEventListener('mouseenter', () => handleEnter(idx));
      step.addEventListener('mouseleave', handleLeave);
      // Touch events
      step.addEventListener('touchstart', (e) => { e.preventDefault(); handleEnter(idx); });
      step.addEventListener('touchend', (e) => { e.preventDefault(); handleLeave(); });
      // Keyboard focus
      step.addEventListener('focus', () => handleEnter(idx));
      step.addEventListener('blur', handleLeave);
    });

  }, 100);
}

function setupSVGSimulator(successTrace) {
  const btnSuccess = document.getElementById('toggle-success');
  const btnLeak = document.getElementById('toggle-leak');
  const btnLoop = document.getElementById('toggle-loop');

  const lineCheck = document.getElementById('line-check');
  const lineRun = document.getElementById('line-run');
  const outBox = document.getElementById('out-box');
  const outText = document.getElementById('out-text');

  const pathLeak = document.getElementById('path-leak');
  const textLeak = document.getElementById('text-leak');
  const pathLoop = document.getElementById('path-loop');
  const textLoop = document.getElementById('text-loop');

  const traceOutput = document.getElementById('sandbox-trace-output');

  function reset() {
    [btnSuccess, btnLeak, btnLoop].forEach(b => {
      b.classList.remove('bg-green-900/50', 'text-green-400', 'border-green-700', 'bg-rose-900/50', 'text-rose-400', 'border-rose-700', 'bg-amber-900/50', 'text-amber-400', 'border-amber-700');
      b.classList.add('bg-slate-800', 'text-slate-400', 'border-slate-700');
    });
    lineCheck.setAttribute('stroke', '#475569');
    lineCheck.setAttribute('marker-end', 'url(#arrow)');
    lineRun.setAttribute('stroke', '#475569');
    lineRun.setAttribute('marker-end', 'url(#arrow)');
    outBox.setAttribute('stroke', '#475569');
    outText.textContent = 'Pending';
    
    pathLeak.setAttribute('opacity', '0');
    textLeak.setAttribute('opacity', '0');
    pathLoop.setAttribute('opacity', '0');
    textLoop.setAttribute('opacity', '0');
  }

  btnSuccess.addEventListener('click', () => {
    reset();
    btnSuccess.classList.remove('bg-slate-800', 'text-slate-400', 'border-slate-700');
    btnSuccess.classList.add('bg-green-900/50', 'text-green-400', 'border-green-700');
    
    lineCheck.setAttribute('stroke', '#10b981');
    lineRun.setAttribute('stroke', '#10b981');
    outBox.setAttribute('stroke', '#10b981');
    outText.textContent = 'Success';
    
    traceOutput.innerHTML = successTrace;
    traceOutput.className = "text-green-400 whitespace-pre-wrap";
  });

  btnLeak.addEventListener('click', () => {
    reset();
    btnLeak.classList.remove('bg-slate-800', 'text-slate-400', 'border-slate-700');
    btnLeak.classList.add('bg-rose-900/50', 'text-rose-400', 'border-rose-700');
    
    lineCheck.setAttribute('stroke', '#ef4444');
    lineCheck.setAttribute('marker-end', 'url(#arrow-red)');
    outBox.setAttribute('stroke', '#475569');
    outText.textContent = 'Halted';
    
    pathLeak.setAttribute('opacity', '1');
    textLeak.setAttribute('opacity', '1');

    traceOutput.innerHTML = "[ERROR] Two-Pass Linear Checker Failed.<br>UnconsumedResource(0): Node 0 was allocated but never explicitly dropped.<br><br>Agent halted before runtime.";
    traceOutput.className = "text-rose-400 whitespace-pre-wrap";
  });

  btnLoop.addEventListener('click', () => {
    reset();
    btnLoop.classList.remove('bg-slate-800', 'text-slate-400', 'border-slate-700');
    btnLoop.classList.add('bg-amber-900/50', 'text-amber-400', 'border-amber-700');
    
    lineCheck.setAttribute('stroke', '#10b981');
    lineRun.setAttribute('stroke', '#ef4444');
    lineRun.setAttribute('marker-end', 'url(#arrow-red)');
    outBox.setAttribute('stroke', '#ef4444');
    outText.textContent = 'Trapped';
    
    pathLoop.setAttribute('opacity', '1');
    textLoop.setAttribute('opacity', '1');

    traceOutput.innerHTML = "Step 0: executed Alloc node<br>Step 1: executed Recursive jump node<br>Step 2: executed Recursive jump node<br>...<br>[TRAP] Wasm Engine Trap: Fuel Exhaustion.<br>Infinite loop detected and safely killed at 10,000 instructions.";
    traceOutput.className = "text-amber-400 whitespace-pre-wrap";
  });
}

function setupCalculator() {
  const models = {
    'gemini': { out: 0.40 / 1000000 },
    'claude': { out: 15.00 / 1000000 },
    'gpt4o': { out: 10.00 / 1000000 }
  };

  const selModel = document.getElementById('calc-model');
  const inCalls = document.getElementById('calc-calls');
  const inSteps = document.getElementById('calc-steps');
  
  const valCalls = document.getElementById('val-calls');
  const valSteps = document.getElementById('val-steps');
  
  const outSavings = document.getElementById('out-savings');
  const outPy = document.getElementById('out-python-cost');
  const outIml = document.getElementById('out-iml-cost');

  // Multiplier estimated: Python ~78 tokens per node logic, IML ~15 tokens per node logic
  const pyTokensPerStep = 78;
  const imlTokensPerStep = 15;

  function update() {
    const calls = parseInt(inCalls.value, 10);
    const steps = parseInt(inSteps.value, 10);
    const costPerToken = models[selModel.value].out;
    
    valCalls.textContent = calls.toLocaleString();
    valSteps.textContent = steps.toLocaleString();
    
    const monthlyCalls = calls * 30;
    
    const pyTotalTokens = monthlyCalls * (steps * pyTokensPerStep);
    const imlTotalTokens = monthlyCalls * (steps * imlTokensPerStep);
    
    const pyCost = pyTotalTokens * costPerToken;
    const imlCost = imlTotalTokens * costPerToken;
    const savings = pyCost - imlCost;
    
    const fmt = new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 });
    
    outPy.textContent = fmt.format(pyCost);
    outIml.textContent = fmt.format(imlCost);
    outSavings.textContent = fmt.format(savings);
  }

  selModel.addEventListener('change', update);
  inCalls.addEventListener('input', update);
  inSteps.addEventListener('input', update);
  
  update();
}

document.addEventListener('DOMContentLoaded', init);
