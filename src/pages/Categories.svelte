<script lang="ts">
  import { t } from '../lib/i18n';
  import { categories, addCategory, removeCategory } from '../lib/stores/settings';

  let newCategory = $state('');

  function handleAdd() {
    const cat = newCategory.trim();
    if (cat && !$categories.includes(cat)) {
      addCategory(cat);
      newCategory = '';
    }
  }

  function handleRemove(cat: string) {
    removeCategory(cat);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      handleAdd();
    }
  }

  const wallhavenHot = ['Nature', 'Minimal', 'Abstract', 'Anime', 'City', 'Space', 'Ocean', 'Mountain', 'Forest', 'Architecture', 'Cyberpunk', 'Dark'];
  const pexelsHot = ['Landscape', 'Sunset', 'Flowers', 'Beach', 'Autumn', 'Winter', 'Wildlife', 'Roads', 'Mountains', 'Stars'];
  const unsplashHot = ['Travel', 'Textures', 'Experimental', 'Film', '3D Renders', 'Interior', 'Street Photography', 'Minimalism'];
</script>

<div class="glass-card">
  <div class="section-header">
    <span class="section-title">{$t('categories.title')}</span>
  </div>

  <!-- Input area -->
  <div class="cat-input-area">
    <div class="cat-input-row">
      <input
        type="text"
        class="cat-input"
        placeholder={$t('categories.placeholder')}
        bind:value={newCategory}
        onkeydown={handleKeydown}
      />
      <button class="cat-add-btn" onclick={handleAdd} disabled={!newCategory.trim()}>
        {$t('categories.add_btn')}
      </button>
    </div>
  </div>

  <!-- User's selected categories -->
  <div class="cat-selected">
    {#each $categories as cat (cat)}
      <div class="cat-tag">
        <span>{cat}</span>
        <span class="remove" onclick={() => handleRemove(cat)}>×</span>
      </div>
    {:else}
      <div class="empty-hint">No categories yet — add some to get personalized wallpapers</div>
    {/each}
  </div>

  <!-- Wallhaven hot -->
  <div class="cat-source-group">
    <div class="cat-source-name">{$t('categories.wallhaven_hot')}</div>
    <div class="cat-suggestions">
      {#each wallhavenHot as tag}
        <span class="cat-suggest" onclick={() => { if (!$categories.includes(tag)) addCategory(tag); }}>{tag}</span>
      {/each}
    </div>
  </div>

  <!-- Pexels hot -->
  <div class="cat-source-group">
    <div class="cat-source-name">{$t('categories.pexels_hot')}</div>
    <div class="cat-suggestions">
      {#each pexelsHot as tag}
        <span class="cat-suggest" onclick={() => { if (!$categories.includes(tag)) addCategory(tag); }}>{tag}</span>
      {/each}
    </div>
  </div>

  <!-- Unsplash hot -->
  <div class="cat-source-group">
    <div class="cat-source-name">{$t('categories.unsplash_hot')}</div>
    <div class="cat-suggestions">
      {#each unsplashHot as tag}
        <span class="cat-suggest" onclick={() => { if (!$categories.includes(tag)) addCategory(tag); }}>{tag}</span>
      {/each}
    </div>
  </div>
</div>

<style>
  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .section-title {
    font-size: 13px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 1.2px;
    color: var(--text-secondary, #9aa0ab);
  }

  .cat-input-area {
    margin-bottom: 20px;
  }

  .cat-input-row {
    display: flex;
    gap: 8px;
  }

  .cat-input {
    flex: 1;
    padding: 10px 16px;
    border-radius: 100px;
    border: 1px solid var(--glass-border, rgba(255,255,255,0.12));
    background: rgba(0,0,0,0.15);
    color: var(--text-primary, #e8eaed);
    font-size: 13px;
    font-family: inherit;
    outline: none;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .cat-input:focus {
    border-color: var(--accent, #5b9cf5);
    box-shadow: 0 0 0 3px var(--accent-soft, rgba(91,156,245,0.15));
  }

  .cat-add-btn {
    padding: 10px 22px;
    border-radius: 100px;
    border: 1px solid var(--accent, #5b9cf5);
    background: var(--accent-soft, rgba(91,156,245,0.15));
    color: var(--accent, #5b9cf5);
    font-size: 13px;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .cat-add-btn:hover:not(:disabled) {
    background: var(--accent, #5b9cf5);
    color: white;
  }

  .cat-add-btn:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .cat-selected {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 20px;
  }

  .cat-tag {
    padding: 6px 16px;
    border-radius: 100px;
    font-size: 12px;
    font-weight: 500;
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.10);
    color: var(--text-secondary, #9aa0ab);
    display: flex;
    align-items: center;
    gap: 6px;
    transition: all 0.3s;
  }

  .cat-tag:hover {
    border-color: rgba(255,255,255,0.2);
    color: var(--text-primary, #e8eaed);
  }

  .cat-tag .remove {
    font-size: 14px;
    opacity: 0.5;
    line-height: 1;
    cursor: pointer;
  }
  .cat-tag .remove:hover { opacity: 1; }

  .empty-hint {
    font-size: 12px;
    color: var(--text-muted, #5f6672);
    padding: 4px 0;
  }

  .cat-source-group {
    margin-bottom: 16px;
  }

  .cat-source-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted, #5f6672);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-bottom: 10px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .cat-source-name::before {
    content: '';
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent, #5b9cf5);
    opacity: 0.6;
  }

  .cat-suggestions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .cat-suggest {
    padding: 5px 13px;
    border-radius: 100px;
    font-size: 11.5px;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.06);
    color: var(--text-muted, #5f6672);
    cursor: pointer;
    transition: all 0.3s;
  }

  .cat-suggest:hover {
    background: var(--accent-soft, rgba(91,156,245,0.15));
    border-color: rgba(91,156,245,0.2);
    color: var(--accent, #5b9cf5);
    transform: translateY(-1px);
  }
</style>
