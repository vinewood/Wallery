<script lang="ts">
  import { t } from '../lib/i18n';
  import { sources, toggleSource, setApiKey } from '../lib/stores/settings';

  let apiKeys: Record<string, string> = $state({});
  let editingKey = $state<string | null>(null);

  function handleToggle(name: string, enabled: boolean) {
    toggleSource(name, enabled);
  }

  function startEditKey(source: string) {
    editingKey = source;
  }

  function saveKey(source: string) {
    const key = apiKeys[source] || '';
    setApiKey(source, key);
    editingKey = null;
  }

  // Source config: name suffix, colors, needs key
  const sourceMeta: Record<string, { icon: string; bg: string; color: string }> = {
    bing: { icon: 'B', bg: 'linear-gradient(135deg,#00a4ef,#0078d4)', color: 'white' },
    wallhaven: { icon: 'WH', bg: 'linear-gradient(135deg,#1a1a2e,#16213e)', color: '#5b9cf5' },
    nasa: { icon: 'NASA', bg: 'linear-gradient(135deg,#0b3d91,#000000)', color: 'white' },
    pexels: { icon: 'Px', bg: 'linear-gradient(135deg,#06A77D,#023020)', color: 'white' },
    unsplash: { icon: 'Un', bg: 'linear-gradient(135deg,#111,#333)', color: 'white' },
  };

  const apiKeyPlaceholders: Record<string, string> = {
    pexels: $t('sources.pexels_placeholder'),
    unsplash: $t('sources.unsplash_placeholder'),
    nasa: $t('sources.nasa_placeholder'),
  };
</script>

<div class="glass-card">
  <div class="sources-grid">
    {#each $sources as src (src.name)}
      <div class="source-item" class:enabled={src.enabled}>
        <div class="source-icon-wrap glass-style" style="background:{sourceMeta[src.name]?.bg || 'linear-gradient(135deg,#555,#333)'}">
          <span class="source-icon-text" style="color:{sourceMeta[src.name]?.color || 'white'}">{sourceMeta[src.name]?.icon || '?'}</span>
        </div>
        <div class="source-info">
          <div class="source-name">
            {src.display_name}
            {#if ['bing', 'wallhaven', 'nasa'].includes(src.name)}
              <span class="source-badge default">{$t('sources.default_badge')}</span>
            {/if}
          </div>
          <div class="source-desc">
            {#if src.name === 'bing'}{$t('sources.bing_desc')}
            {:else if src.name === 'wallhaven'}{$t('sources.wallhaven_desc')}
            {:else if src.name === 'nasa'}{$t('sources.nasa_desc')}
            {:else if src.name === 'pexels'}{$t('sources.pexels_desc')}
            {:else if src.name === 'unsplash'}{$t('sources.unsplash_desc')}
            {/if}
          </div>
        </div>
        <button class="toggle" class:on={src.enabled} onclick={() => handleToggle(src.name, !src.enabled)} role="switch" aria-checked={src.enabled}>
          <span class="toggle-knob"></span>
        </button>
      </div>
    {/each}

    <!-- API Keys row -->
    <div class="api-key-row">
      <span class="api-key-label">🔑 {$t('sources.api_key_label')}</span>
      <input type="password" class="api-key-input" placeholder={$t('sources.pexels_placeholder')} />
      <input type="password" class="api-key-input" placeholder={$t('sources.unsplash_placeholder')} />
      <input type="password" class="api-key-input" placeholder={$t('sources.nasa_placeholder')} />
    </div>
  </div>
</div>

<style>
  .sources-grid {
    display: grid;
    gap: 12px;
  }

  .source-item {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 14px 18px;
    border-radius: 12px;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.07);
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    position: relative;
  }

  .source-item::after {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.08), transparent);
  }

  .source-item:hover {
    background: rgba(255,255,255,0.05);
    border-color: rgba(255,255,255,0.12);
    transform: translateY(-1px);
  }

  .source-item.enabled {
    border-color: rgba(91, 156, 245, 0.2);
    background: rgba(91, 156, 245, 0.04);
  }

  .source-icon-wrap {
    width: 42px;
    height: 42px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    background: linear-gradient(145deg, rgba(255,255,255,0.12), rgba(255,255,255,0.04));
    border: 1px solid rgba(255,255,255,0.15);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.1), 0 2px 8px rgba(0,0,0,0.2);
  }

  .source-icon-text {
    font-size: 13px;
    font-weight: 700;
    letter-spacing: -0.3px;
  }

  .source-info { flex: 1; min-width: 0; }

  .source-name {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary, #e8eaed);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .source-badge {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 100px;
    background: var(--accent-soft, rgba(91,156,245,0.15));
    color: var(--accent, #5b9cf5);
    font-weight: 500;
  }
  .source-badge.default {
    background: rgba(255,255,255,0.08);
    color: var(--text-secondary, #9aa0ab);
  }

  .source-desc {
    font-size: 11.5px;
    color: var(--text-muted, #5f6672);
    margin-top: 3px;
  }

  .toggle {
    width: 40px;
    height: 22px;
    border-radius: 100px;
    background: rgba(255,255,255,0.08);
    border: 1px solid rgba(255,255,255,0.10);
    cursor: pointer;
    position: relative;
    flex-shrink: 0;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    padding: 0;
  }

  .toggle.on {
    background: linear-gradient(90deg, var(--accent-soft, rgba(91,156,245,0.15)), rgba(91,156,245,0.35));
    border-color: rgba(91,156,245,0.3);
  }

  .toggle-knob {
    display: block;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: linear-gradient(145deg, #d0d5dc, #8892a0);
    position: absolute;
    top: 2px;
    left: 2px;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 1px 3px rgba(0,0,0,0.3);
  }

  .toggle.on .toggle-knob {
    left: 20px;
    background: linear-gradient(145deg, #fff, #c0d0f0);
    box-shadow: 0 1px 6px rgba(91,156,245,0.4);
  }

  .api-key-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 18px;
    border-radius: 12px;
    background: rgba(0,0,0,0.15);
    border: 1px dashed rgba(255,255,255,0.08);
    flex-wrap: wrap;
  }

  .api-key-label {
    font-size: 12px;
    color: var(--text-muted, #5f6672);
    white-space: nowrap;
    font-weight: 500;
  }

  .api-key-input {
    flex: 1;
    min-width: 140px;
    padding: 8px 14px;
    border-radius: 8px;
    border: 1px solid rgba(255,255,255,0.08);
    background: rgba(0,0,0,0.2);
    color: var(--text-primary, #e8eaed);
    font-size: 13px;
    font-family: inherit;
    outline: none;
    transition: border-color 0.3s;
  }

  .api-key-input:focus {
    border-color: var(--accent, #5b9cf5);
    box-shadow: 0 0 0 3px var(--accent-soft, rgba(91,156,245,0.15));
  }

  .api-key-input::placeholder {
    color: var(--text-muted, #5f6672);
  }
</style>
