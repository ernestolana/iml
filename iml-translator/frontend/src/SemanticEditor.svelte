<script>
  import { onMount } from 'svelte';

  let nodes = $state([]);
  let error = $state(null);
  let loading = $state(true);

  const API_BASE = ''; // Assumes same origin

  async function fetchAST() {
    error = null;
    loading = true;
    try {
      const response = await fetch(`${API_BASE}/ast`);
      if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
      const data = await response.json();
      nodes = data.nodes || [];
    } catch (err) {
      console.error("Failed to fetch AST:", err);
      error = `Failed to load AST: ${err.message}`;
    } finally {
      loading = false;
    }
  }

  async function saveNode(nodeIndex, updatedText) {
    error = null;
    try {
      const response = await fetch(`${API_BASE}/translate`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          node_index: nodeIndex,
          updated_text: updatedText
        })
      });

      if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
      const data = await response.json();
      nodes = data.nodes || [];
    } catch (err) {
      console.error("Failed to save node:", err);
      error = `Failed to update Node ${nodeIndex}: ${err.message}`;
    }
  }

  onMount(() => {
    fetchAST();
  });
</script>

<div class="editor-container">
  <div class="header">
    <h1>IML Translator IDE</h1>
    <button onclick={fetchAST}>Refresh</button>
  </div>

  {#if loading}
    <p>Loading AST...</p>
  {/if}

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="nodes-list">
    {#each nodes as node, index (index)}
      {#if node.r !== undefined && node.r !== null}
        <div class="node-block">
          <div class="node-header">
            <span>Human Rationale</span>
            <span class="node-badge">Node {index}</span>
          </div>
          <textarea
            bind:value={node.r}
            rows="4"
          ></textarea>
          <div class="actions">
            <button onclick={() => saveNode(index, node.r)}>Update</button>
          </div>
        </div>
      {/if}
    {/each}
    {#if !loading && nodes.filter(n => n.r !== undefined && n.r !== null).length === 0}
      <p class="empty-state">No human_rationale (r) nodes found.</p>
    {/if}
  </div>
</div>

<style>
  .editor-container {
    max-width: 800px;
    margin: 0 auto;
    font-family: system-ui, -apple-system, sans-serif;
  }
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #ccc;
    padding-bottom: 1rem;
    margin-bottom: 1rem;
  }
  .error {
    background-color: #fee;
    color: #c00;
    padding: 1rem;
    border-left: 4px solid #c00;
    margin-bottom: 1rem;
  }
  .nodes-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .node-block {
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 1rem;
    background: #f9f9f9;
  }
  .node-header {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    color: #666;
    margin-bottom: 0.5rem;
  }
  .node-badge {
    font-family: monospace;
    background: #eee;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
  }
  textarea {
    width: 100%;
    box-sizing: border-box;
    font-family: monospace;
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 0.5rem;
  }
  button {
    background: #007acc;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }
  button:hover {
    background: #005f9e;
  }
  .empty-state {
    text-align: center;
    color: #888;
  }
</style>
