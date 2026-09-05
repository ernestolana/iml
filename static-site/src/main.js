import showcaseData from './data/showcase.json';

function init() {
  const app = document.getElementById('app');
  
  const iml = showcaseData.find(d => d.id === 'iml');
  const python = showcaseData.find(d => d.id === 'python');
  const rust = showcaseData.find(d => d.id === 'rust');
  const verbose = showcaseData.find(d => d.id === 'verbose_json');
  const slop = showcaseData.find(d => d.id === 'slop');

  const competitors = [python, rust, verbose, slop];

  const html = `
    <!-- The Bottleneck -->
    <section id="bottleneck" class="py-24 px-4 bg-slate-900 border-t border-slate-800">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl font-bold mb-4 text-white">The AI Bottleneck (Comparisons)</h2>
        <p class="text-slate-400 mb-12 max-w-3xl leading-relaxed">
          LLMs are currently forced to generate token-heavy, legacy human syntax. This wastes context windows, inflates latency, and massively increases generation costs. Below is a simple "Microgrid Power Stabilization" function written in traditional formats vs. IML.
        </p>

        <div class="grid lg:grid-cols-2 gap-8">
          <!-- Competitor Tabs -->
          <div class="bg-slate-950 border border-slate-800 rounded-xl overflow-hidden shadow-2xl flex flex-col">
            <div class="flex border-b border-slate-800 bg-slate-900/50" id="tab-headers">
              ${competitors.map((c, i) => `
                <button class="tab-btn px-4 py-3 text-sm font-medium transition-colors ${i === 0 ? 'text-cyan-400 border-b-2 border-cyan-400 bg-slate-900' : 'text-slate-500 hover:text-slate-300'}" data-target="${c.id}">
                  ${c.name}
                </button>
              `).join('')}
            </div>
            <div class="p-4 flex-1 overflow-auto bg-[#1d1f21]" id="tab-contents">
              ${competitors.map((c, i) => `
                <div class="tab-pane ${i === 0 ? 'block' : 'hidden'}" id="pane-${c.id}">
                  <div class="flex justify-between items-center mb-4 text-xs font-mono text-slate-500 border-b border-slate-800 pb-2">
                    <span>${c.tokens} Tokens</span>
                    <span>${c.bytes} Bytes</span>
                  </div>
                  <pre><code class="language-${c.id === 'python' ? 'python' : c.id === 'rust' ? 'rust' : c.id === 'slop' ? 'lisp' : 'json'}">${c.code}</code></pre>
                </div>
              `).join('')}
            </div>
          </div>

          <!-- IML Showcase -->
          <div class="bg-slate-950 border border-cyan-900/50 rounded-xl overflow-hidden shadow-[0_0_40px_rgba(6,182,212,0.1)] flex flex-col relative">
            <div class="absolute top-0 right-0 bg-cyan-500 text-slate-950 text-xs font-bold px-3 py-1 rounded-bl-lg">WINNER</div>
            <div class="px-6 py-4 border-b border-slate-800 bg-slate-900/80 backdrop-blur">
              <h3 class="font-semibold text-cyan-400">${iml.name}</h3>
            </div>
            <div class="p-6 flex-1 overflow-auto bg-[#1d1f21]">
              <div class="flex justify-between items-center mb-4 text-xs font-mono text-cyan-500 border-b border-slate-800 pb-2">
                <span>${iml.tokens} Tokens</span>
                <span>${iml.bytes} Bytes</span>
              </div>
              <pre><code class="language-json">${iml.code}</code></pre>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- The Solution -->
    <section id="solution" class="py-24 px-4 bg-slate-950">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl font-bold mb-4 text-white">The IML Solution (Dual-State Architecture)</h2>
        <p class="text-slate-400 mb-12 max-w-3xl leading-relaxed">
          IML separates code into two distinct states: a <strong>Machine-Native AST</strong> for strict, linear AI generation, and a <strong>Semantic Overlay</strong> for human-readable debugging. They are mathematically equivalent and bind bidirectionally.
        </p>

        <div class="grid md:grid-cols-2 gap-8">
          <div class="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-xl">
            <h4 class="text-sm font-bold text-slate-500 uppercase tracking-wider mb-4">Semantic English Overlay (Human)</h4>
            <pre class="font-mono text-sm text-amber-200 bg-black/40 p-4 rounded-lg overflow-x-auto">${iml.overlay}</pre>
          </div>
          <div class="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-xl">
            <h4 class="text-sm font-bold text-slate-500 uppercase tracking-wider mb-4">Machine-Native AST (AI)</h4>
            <pre><code class="language-json text-sm">${iml.code}</code></pre>
          </div>
        </div>
      </div>
    </section>

    <!-- Execution & Benchmarks -->
    <section id="benchmarks" class="py-24 px-4 bg-slate-900 border-t border-slate-800">
      <div class="max-w-6xl mx-auto">
        <div class="grid lg:grid-cols-2 gap-16 items-center">
          <div>
            <h2 class="text-3xl font-bold mb-4 text-white">Execution & Safety</h2>
            <p class="text-slate-400 mb-6 leading-relaxed">
              Because IML relies on a strictly linear DAG, ownership constraints and memory drops are verified statically in a single pass. 
            </p>
            <div class="bg-black/50 border border-slate-800 rounded-xl p-6 font-mono text-sm shadow-inner">
              <div class="text-slate-500 mb-2">// Sandbox Trace Output</div>
              <div class="text-green-400 whitespace-pre-wrap">${iml.trace}</div>
            </div>
          </div>
          
          <div>
            <h2 class="text-3xl font-bold mb-4 text-white">Cost Reduction Metrics</h2>
            <div class="space-y-6">
              ${competitors.map(c => {
                const saving = Math.round((1 - (iml.tokens / c.tokens)) * 100);
                return `
                  <div>
                    <div class="flex justify-between text-sm mb-2">
                      <span class="text-slate-300 font-medium">vs ${c.name}</span>
                      <span class="text-cyan-400 font-bold">${saving}% fewer tokens</span>
                    </div>
                    <div class="w-full bg-slate-800 rounded-full h-2.5">
                      <div class="bg-cyan-500 h-2.5 rounded-full" style="width: ${saving}%"></div>
                    </div>
                  </div>
                `;
              }).join('')}
            </div>
          </div>
        </div>
      </div>
    </section>
  `;

  app.innerHTML = html;

  // Setup tabs
  const headers = document.querySelectorAll('.tab-btn');
  const panes = document.querySelectorAll('.tab-pane');

  headers.forEach(btn => {
    btn.addEventListener('click', () => {
      headers.forEach(h => {
        h.classList.remove('text-cyan-400', 'border-b-2', 'border-cyan-400', 'bg-slate-900');
        h.classList.add('text-slate-500');
      });
      panes.forEach(p => p.classList.add('hidden'));

      btn.classList.add('text-cyan-400', 'border-b-2', 'border-cyan-400', 'bg-slate-900');
      btn.classList.remove('text-slate-500');
      
      const target = btn.getAttribute('data-target');
      document.getElementById('pane-' + target).classList.remove('hidden');
    });
  });

  // Re-run prism highlighting
  if (window.Prism) {
    window.Prism.highlightAll();
  }
}

document.addEventListener('DOMContentLoaded', init);
