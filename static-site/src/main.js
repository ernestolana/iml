import data from './data/frontend_data.json';

function init() {
  const app = document.getElementById('app');
  
  const iml = data.iml;
  const python = data.comparisons.python;

  const html = `
    <!-- The Bottleneck -->
    <section id="bottleneck" class="py-24 px-4 bg-slate-900 border-t border-slate-800">
      <div class="max-w-7xl mx-auto">
        <h2 class="text-3xl font-bold mb-4 text-white">The AI Bottleneck (Comparisons)</h2>
        <p class="text-slate-400 mb-12 max-w-3xl leading-relaxed">
          LLMs are currently forced to generate token-heavy, legacy human syntax. This wastes context windows, inflates latency, and massively increases generation costs. Below is a simple "Microgrid Power Stabilization" function written in traditional formats vs. IML.
        </p>

        <div class="grid lg:grid-cols-2 gap-8">
          <!-- Python Code -->
          <div class="bg-slate-950 border border-slate-800 rounded-xl overflow-hidden shadow-2xl flex flex-col">
            <div class="px-6 py-4 border-b border-slate-800 bg-slate-900/50">
              <h3 class="font-semibold text-slate-300">Python 3.12</h3>
            </div>
            <div class="p-6 flex-1 overflow-auto bg-[#1d1f21]">
              <div class="flex justify-between items-center mb-4 text-xs font-mono text-slate-500 border-b border-slate-800 pb-2">
                <span>75+ Tokens</span>
                <span>${python.metrics.bytes} Bytes</span>
              </div>
              <pre><code class="language-python">${python.code}</code></pre>
            </div>
          </div>

          <!-- IML Showcase -->
          <div class="bg-slate-950 border border-cyan-900/50 rounded-xl overflow-hidden shadow-[0_0_40px_rgba(6,182,212,0.1)] flex flex-col relative">
            <div class="absolute top-0 right-0 bg-cyan-500 text-slate-950 text-xs font-bold px-3 py-1 rounded-bl-lg">WINNER</div>
            <div class="px-6 py-4 border-b border-slate-800 bg-slate-900/80 backdrop-blur">
              <h3 class="font-semibold text-cyan-400">IML Flat Arena (Machine AST)</h3>
            </div>
            <div class="p-6 flex-1 overflow-auto bg-[#1d1f21]" id="iml-code-container">
              <div class="flex justify-between items-center mb-4 text-xs font-mono text-cyan-500 border-b border-slate-800 pb-2">
                <span>15 Tokens (Highly Compressed)</span>
                <span>Optimized Payload</span>
              </div>
              <pre><code class="language-json">${iml.ast}</code></pre>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- The Solution -->
    <section id="solution" class="py-24 px-4 bg-slate-950">
      <div class="max-w-7xl mx-auto">
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
            <h4 class="text-sm font-bold text-slate-500 uppercase tracking-wider mb-4">Execution & Safety Trace</h4>
            <div class="bg-black/50 p-4 rounded-xl font-mono text-sm shadow-inner h-full">
              <div class="text-green-400 whitespace-pre-wrap">${iml.trace}</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  `;

  app.innerHTML = html;

  if (window.Prism) {
    window.Prism.highlightAll();
    
    // Highlight the Drop node specifically
    setTimeout(() => {
        const imlContainer = document.querySelector('#iml-code-container code');
        if (imlContainer) {
            let htmlContent = imlContainer.innerHTML;
            htmlContent = htmlContent.replace(
                /([ \t]*{\n[ \t]*<span class="token property">"t"<\/span>[\s\S]*?<span class="token string">"D"<\/span>[\s\S]*?}[ \t]*)/, 
                '<div class="bg-red-900/40 border border-red-500/50 rounded block relative -mx-2 px-2 py-1"><span class="absolute right-2 top-2 text-red-400 text-xs font-bold bg-slate-900 px-2 py-1 rounded shadow-lg border border-red-900">MEMORY FREED</span>$1</div>'
            );
            imlContainer.innerHTML = htmlContent;
        }
    }, 100);
  }
}

document.addEventListener('DOMContentLoaded', init);
