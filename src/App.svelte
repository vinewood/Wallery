<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from './lib/i18n';
  import { sources, selectedSource, selectedCategory, browsePage, activeTab, categories, favorites, loadSourcesStatus, loadSettings, browseSource, setWallpaperFrom, loadFavorites, addFavorite, removeFavorite } from './lib/stores/settings';
  import type { WallpaperItem, FavoriteItem } from './lib/stores/settings';
  import { invoke } from '@tauri-apps/api/core';
  import SourcesPage from './pages/Sources.svelte';
  import CategoriesPage from './pages/Categories.svelte';
  import SchedulePage from './pages/Schedule.svelte';
  import AboutPage from './pages/About.svelte';

  let currentTab = $state('browse');
  let wallpapers: WallpaperItem[] = $state([]);
  let loading = $state(false);
  let settingIdx = $state<number | null>(null);
  let downloadingIdx = $state<number | null>(null);
  let toast = $state<{ msg: string; type: 'ok' | 'err' } | null>(null);
  let toastTimer: ReturnType<typeof setTimeout> | null = null;
  const totalPages = 5;

  $effect(() => { activeTab.set(currentTab); });

  const brandColors: Record<string, string> = {
    bing: '#008373', wallhaven: '#4d7cff', nasa: '#0b3d91', pexels: '#05a081', unsplash: '#000000',
  };
  const brandIcons: Record<string, string> = {
    bing: 'B', wallhaven: 'WH', nasa: 'N', pexels: 'Px', unsplash: 'Un',
  };

  let chipCategories: string[] = $state([]);
  $effect(() => {
    if ($selectedSource) {
      chipCategories = ($sources.find(s => s.name === $selectedSource)?.hot_categories || []).slice(0, 12);
    } else { chipCategories = $categories.slice(0, 12); }
  });

  // On source/category/page change → reload
  $effect(() => {
    if (currentTab === 'browse') {
      const src = $selectedSource;
      if (src) { loadWallpapers(src, $selectedCategory, $browsePage); } else { wallpapers = []; }
    }
  });

  function showToast(msg: string, type: 'ok' | 'err' = 'ok') {
    if (toastTimer) clearTimeout(toastTimer);
    toast = { msg, type };
    toastTimer = setTimeout(() => { toast = null; }, 5000);
  }

  async function loadWallpapers(src: string, cat: string, page: number) {
    loading = true;
    const items = await browseSource(src, cat, page);
    if (items.length > 0) { wallpapers = items; }
    else { wallpapers = genGradients(src, cat, page, 21); }
    loading = false;
  }

  function genGradients(s: string, cat: string, page: number, count: number): WallpaperItem[] {
    const palette: Record<string, string[]> = {
      bing: ['#008373','#004d40','#00695c','#388e3c','#1b5e20','#2e7d32'],
      wallhaven: ['#4d7cff','#1565c0','#1976d2','#1e88e5','#0d47a1','#42a5f5'],
      nasa: ['#0b3d91','#1a237e','#283593','#303f9f','#3949ab','#5c6bc0'],
      pexels: ['#05a081','#00695c','#00796b','#00897b','#26a69a','#4db6ac'],
      unsplash: ['#37474f','#455a64','#546e7a','#607d8b','#78909c','#90a4ae'],
    };
    const c = palette[s] || palette.wallhaven;
    return Array.from({ length: count }, (_, i) => {
      const hue = ((page - 1) * count + i) * 31 % 360;
      return { id: `${s}-${cat}-${page}-${i}`, url: '', thumb: '', title: `${s} #${(page - 1) * count + i + 1}`, source: s, category: cat, gradient: `linear-gradient(135deg, ${c[i % c.length]}, hsl(${hue},55%,22%))` };
    });
  }

  function switchTab(tab: string) { currentTab = tab; }

  function selectSource(name: string) {
    if ($selectedSource === name) return;
    selectedSource.set(name || '');
    selectedCategory.set('all');
    browsePage.set(1);
    // Save preference
    if (name) invoke('set_last_selected_source', { source: name }).catch(() => {});
  }

  function selectCategory(cat: string) {
    selectedCategory.set(cat);
    browsePage.set(1);
  }

  function goToPage(p: number) {
    if (p < 1 || p > totalPages) return;
    browsePage.set(p);
    document.querySelector('.browse-area')?.scrollTo(0, 0);
  }

  function getPageNumbers(): (number | string)[] {
    const cur = $browsePage;
    if (totalPages <= 7) return Array.from({ length: totalPages }, (_, i) => i + 1);
    const pages: (number | string)[] = [1];
    if (cur > 3) pages.push('…');
    for (let i = Math.max(2, cur - 1); i <= Math.min(totalPages - 1, cur + 1); i++) pages.push(i);
    if (cur < totalPages - 2) pages.push('…');
    pages.push(totalPages);
    return pages;
  }

  async function refreshWallpapers() {
    await loadWallpapers($selectedSource || 'wallhaven', $selectedCategory, $browsePage);
  }

  async function handleSetWallpaper(item: WallpaperItem, idx: number) {
    settingIdx = idx;
    try {
      if (item.url) {
        const ok = await setWallpaperFrom(item.url, item.source);
        if (ok) showToast(`已设为壁纸 (${item.source})`, 'ok');
        else showToast('设为壁纸失败', 'err');
      } else {
        await invoke('next_wallpaper');
        showToast('已切换壁纸', 'ok');
      }
    } catch {
      showToast('设置失败', 'err');
    } finally {
      settingIdx = null;
    }
  }

  async function handleDownload(item: WallpaperItem, idx: number) {
    downloadingIdx = idx;
    try {
      if (item.url) {
        const w = window.screen.width;
        const h = window.screen.height;
        const result: string = await invoke('download_wallpaper_url', {
          url: item.url, source: item.source, screenWidth: w, screenHeight: h,
        });
        showToast(`✓ 已下载: ${item.source}`, 'ok');
      } else {
        await invoke('save_current_wallpaper');
        showToast('✓ 已保存', 'ok');
      }
    } catch { showToast('✗ 下载失败', 'err'); }
    finally { downloadingIdx = null; }
  }

  // Favorites
  async function handleToggleFav(e: Event, item: WallpaperItem) {
    e.stopPropagation();
    const favId = item.id || `${item.source}-${Date.now()}`;
    const isFav = $favorites.some(f => f.id === favId);
    if (isFav) {
      await removeFavorite(favId);
      showToast('已取消收藏', 'ok');
    } else {
      const ok = await addFavorite({
        id: favId, url: item.url, thumbnail: item.thumb || item.url,
        source: item.source, display_name: item.title,
        attribution: item.title, source_page: item.source_page || '',
      });
      if (ok) showToast('已收藏', 'ok');
      else showToast('收藏失败（可能已存在）', 'err');
    }
  }

  function isFav(id: string): boolean { return $favorites.some(f => f.id === (id || '')); }

  // Init: load cache first, then live data
  onMount(async () => {
    // Load cached wallpapers first for instant display
    try {
      const cached: any[] = await invoke('get_wallpaper_cache');
      if (cached.length > 0) {
        wallpapers = cached.map((c: any) => ({
          id: c.id, url: c.url, thumb: c.thumbnail, title: c.attribution,
          source: c.source, category: 'all', gradient: '',
          source_page: c.source_page,
        }));
      }
    } catch { /* no cache */ }

    try {
      await loadSourcesStatus();
      await loadSettings();
      await loadFavorites();

      // Restore last selected source
      const lastSrc: string = await invoke('get_last_selected_source');
      if (lastSrc && $sources.some(s => s.name === lastSrc)) {
        selectedSource.set(lastSrc);
      } else if ($sources.length > 0) {
        selectedSource.set($sources[0].name);
      }
      selectedCategory.set('all');
      browsePage.set(1);

      // If cache loaded, now refresh with live data
      if (wallpapers.length > 0) refreshWallpapers();
    } catch (e) { console.warn('Init error:', e); }
  });
</script>

<div class="app">
  <!-- Title Bar -->
  <div class="title-bar" data-tauri-drag-region>
    <div class="title-left">
      <img src="/cube_icon.png" alt="W" class="title-icon">
    </div>
    <div class="title-center">
      <span class="title-dot-mid"></span>
      幕间 · Wallery
      <span class="title-dot-mid"></span>
    </div>
    <div class="title-right">
      <span class="lang-badge">zh-CN</span>
    </div>
  </div>

  <!-- Tab Bar -->
  <nav class="tab-bar">
    <button class="tab-item" class:active={currentTab === 'browse'} onclick={() => switchTab('browse')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><path d="M21 15l-5-5L5 21"/></svg>浏览
    </button>
    <button class="tab-item" class:active={currentTab === 'favorites'} onclick={() => switchTab('favorites')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M20.84 4.61a5.5 5.5 0 00-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 00-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 000-7.78z"/></svg>收藏
    </button>
    <button class="tab-item" class:active={currentTab === 'sources'} onclick={() => switchTab('sources')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="4" y="4" width="7" height="7" rx="1.5"/><rect x="13" y="4" width="7" height="7" rx="1.5"/><rect x="4" y="13" width="7" height="7" rx="1.5"/><rect x="13" y="13" width="7" height="7" rx="1.5"/></svg>{$t('tabs.sources')}
    </button>
    <button class="tab-item" class:active={currentTab === 'categories'} onclick={() => switchTab('categories')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M20.59 13.41l-7.17 7.17a2 2 0 01-2.83 0L2 12V2h10l8.59 8.59a2 2 0 010 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/></svg>{$t('tabs.categories')}
    </button>
    <button class="tab-item" class:active={currentTab === 'schedule'} onclick={() => switchTab('schedule')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>{$t('tabs.schedule')}
    </button>
    <button class="tab-item" class:active={currentTab === 'about'} onclick={() => switchTab('about')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>{$t('tabs.about')}
    </button>
  </nav>

  <div class="content">
    {#if currentTab === 'browse'}
      <div class="browse-header">
        <select class="browse-source-select" value={$selectedSource} onchange={(e) => { const v = (e.target as HTMLSelectElement).value; selectSource(v || ($sources[0]?.name || '')); }}>
          {#each $sources as src}
            <option value={src.name} style={src.name === $selectedSource ? `border-left:3px solid ${brandColors[src.name] || '#888'}` : ''}>{brandIcons[src.name] || '?'} {src.display_name}</option>
          {/each}
        </select>
        <span class="browse-total">共 {wallpapers.length} 张</span>
        <button class="refresh-btn" class:loading={loading} onclick={refreshWallpapers} title="刷新">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 11-2.12-9.36L23 10"/></svg>
        </button>
      </div>

      <div class="cat-chips">
        <button class="chip" class:active={$selectedCategory === 'all'} onclick={() => selectCategory('all')}>全部</button>
        {#each chipCategories as cat}
          <button class="chip" class:active={$selectedCategory === cat} onclick={() => selectCategory(cat)}>{cat}</button>
        {/each}
      </div>

      {#if loading}<div class="loading-bar"><div class="loading-dot"></div>正在从 {$sources.find(s => s.name === ($selectedSource || ''))?.display_name || '壁纸源'} 加载…</div>{/if}

      <div class="browse-area">
        <div class="wp-grid">
          {#if loading}
            {#each Array(21) as _, i}
              <div class="wp-card skeleton"><div class="skeleton-pulse"></div></div>
            {/each}
          {:else}
            {#each wallpapers as item, idx (item.id)}
              <div class="wp-card">
                {#if isFav(item.id)}<div class="fav-badge">★</div>{/if}
                {#if item.thumb}
                  <img class="wp-thumb" src={item.thumb} alt={item.title} loading="lazy" />
                {:else}
                  <div class="wp-thumb-placeholder" style="background:{item.gradient}">{brandIcons[item.source] || 'W'}</div>
                {/if}
                <div class="wp-overlay">
                  <span class="wp-source-tag">{item.source}</span>
                  <div class="wp-actions">
                    <button class="wp-btn" title="设为壁纸" disabled={settingIdx === idx} onclick={() => handleSetWallpaper(item, idx)}>
                      {#if settingIdx === idx}
                        <div class="btn-spinner"></div>
                      {:else}
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><path d="M21 15l-5-5L5 21"/></svg>
                      {/if}
                    </button>
                    <button class="wp-btn" class:faved={isFav(item.id)} title="收藏" onclick={(e) => handleToggleFav(e, item)}>
                      <svg width="16" height="16" viewBox="0 0 24 24" fill={isFav(item.id) ? 'currentColor' : 'none'} stroke="currentColor" stroke-width="2"><path d="M20.84 4.61a5.5 5.5 0 00-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 00-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 000-7.78z"/></svg>
                    </button>
                    <button class="wp-btn" title="下载" disabled={downloadingIdx === idx} onclick={() => handleDownload(item, idx)}>
                      {#if downloadingIdx === idx}
                        <div class="btn-spinner"></div>
                      {:else}
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                      {/if}
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          {/if}
        </div>

        <div class="pagination-bar">
          <button class="page-btn" disabled={$browsePage <= 1} onclick={() => goToPage($browsePage - 1)}>‹ 上一页</button>
          {#each getPageNumbers() as p}
            {#if p === '…'}<span class="page-info">…</span>
            {:else}<button class="page-btn" class:active={$browsePage === p} onclick={() => goToPage(p as number)}>{p}</button>
            {/if}
          {/each}
          <button class="page-btn" disabled={$browsePage >= totalPages} onclick={() => goToPage($browsePage + 1)}>下一页 ›</button>
        </div>
      </div>

    {:else if currentTab === 'favorites'}
      <div class="browse-header"><h3 style="font-size:14px;font-weight:500;color:var(--text-secondary);">我的收藏 ({$favorites.length})</h3></div>
      <div class="browse-area">
        {#if $favorites.length === 0}
          <div class="empty-state">还没有收藏壁纸，浏览时点击 ♡ 按钮添加</div>
        {:else}
          <div class="wp-grid">
            {#each $favorites as fav (fav.id)}
              <div class="wp-card">
                <img class="wp-thumb" src={fav.thumbnail} alt={fav.attribution} loading="lazy" />
                <div class="wp-overlay">
                  <span class="wp-source-tag">{fav.source}</span>
                  <div class="wp-actions">
                    <button class="wp-btn" title="设为壁纸" onclick={() => setWallpaperFrom(fav.url, fav.source)}>
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><path d="M21 15l-5-5L5 21"/></svg>
                    </button>
                    <button class="wp-btn faved" title="取消收藏" onclick={async () => { await removeFavorite(fav.id); showToast('已取消收藏', 'ok'); }}>
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2"><path d="M20.84 4.61a5.5 5.5 0 00-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 00-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 000-7.78z"/></svg>
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>

    {:else if currentTab === 'sources'}
      <div class="tab-scroll"><SourcesPage /></div>
    {:else if currentTab === 'categories'}
      <div class="tab-scroll"><CategoriesPage /></div>
    {:else if currentTab === 'schedule'}
      <div class="tab-scroll"><SchedulePage /></div>
    {:else if currentTab === 'about'}
      <div class="tab-scroll"><AboutPage /></div>
    {/if}
  </div>
</div>

<!-- Toast -->
{#if toast}
  <div class="toast" class:err={toast.type === 'err'}>{toast.msg}</div>
{/if}

<style>
  .app { max-width: 1440px; margin: 0 auto; padding: 12px 28px 0; position: relative; z-index: 1; }

  /* Title bar with icon */
  .title-bar { display: flex; align-items: center; padding: 10px 0 12px; gap: 12px; border-bottom: 1px solid var(--glass-border); }
  .title-left { display: flex; align-items: center; }
  .title-icon { width: 22px; height: 22px; border-radius: 6px; }
  .title-center { flex: 1; text-align: center; font-size: 13px; color: var(--text-muted); letter-spacing: 0.5px; display: flex; align-items: center; justify-content: center; gap: 8px; }
  .title-dot-mid { width: 6px; height: 6px; border-radius: 50%; background: var(--accent); display: inline-block; }
  .title-right { display: flex; align-items: center; }
  .lang-badge { font-size: 11px; color: var(--text-muted); padding: 3px 10px; border-radius: 20px; border: 1px solid var(--glass-border); background: var(--glass-bg); }

  .tab-bar { display: flex; gap: 2px; padding: 10px 0 0; border-bottom: 1px solid var(--glass-border); margin-bottom: 18px; }
  .tab-item { padding: 10px 18px; font-size: 13px; color: var(--text-muted); border: none; background: none; cursor: pointer; font-family: inherit; border-bottom: 2px solid transparent; display: flex; align-items: center; gap: 6px; transition: color 0.2s; margin-bottom: -1px; white-space: nowrap; }
  .tab-item:hover { color: var(--text-secondary); }
  .tab-item.active { color: var(--text-primary); border-bottom-color: var(--accent); }

  .content { min-height: 400px; }

  .browse-header { display: flex; align-items: center; gap: 12px; margin-bottom: 12px; }
  .browse-source-select { padding: 7px 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: var(--glass-bg); font-size: 12px; color: var(--text-primary); outline: none; font-family: inherit; cursor: pointer; min-width: 180px; }
  .browse-source-select:focus { border-color: var(--accent); }
  .browse-source-select option { background: #1e2730; color: #e8eaed; padding: 6px; }
  .browse-total { font-size: 11px; color: var(--text-muted); }

  .refresh-btn { margin-left: auto; width: 32px; height: 32px; border-radius: 50%; border: 1px solid var(--glass-border); background: var(--glass-bg); color: var(--text-secondary); cursor: pointer; display: flex; align-items: center; justify-content: center; transition: all 0.2s; }
  .refresh-btn:hover { background: var(--glass-bg-hover); color: var(--text-primary); }
  .refresh-btn.loading svg { animation: spin 1s linear infinite; }
  @keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

  .cat-chips { display: flex; gap: 6px; overflow-x: auto; margin-bottom: 16px; }
  .cat-chips::-webkit-scrollbar { height: 3px; }
  .cat-chips::-webkit-scrollbar-thumb { background: var(--glass-border); border-radius: 2px; }
  .chip { padding: 5px 14px; border-radius: var(--r-pill); border: 1px solid var(--glass-border); background: var(--glass-bg); font-size: 12px; color: var(--text-secondary); cursor: pointer; white-space: nowrap; transition: all 0.2s; font-family: inherit; }
  .chip:hover { background: var(--glass-bg-hover); color: var(--text-primary); }
  .chip.active { background: var(--accent-soft); border-color: var(--accent); color: var(--accent); }

  .loading-bar { display: flex; align-items: center; gap: 10px; padding: 10px 14px; margin-bottom: 14px; border-radius: 10px; background: var(--accent-soft); border: 1px solid rgba(91,156,245,0.2); font-size: 12px; color: var(--accent); }
  .loading-dot { width: 8px; height: 8px; border-radius: 50%; background: var(--accent); animation: pulse 1.2s ease-in-out infinite; }
  @keyframes pulse { 0%,100%{opacity:0.3;transform:scale(1)} 50%{opacity:1;transform:scale(1.3)} }

  .empty-state { display: flex; align-items: center; justify-content: center; height: 30vh; font-size: 13px; color: var(--text-muted); }

  /* Scrollable tabs container */
  .tab-scroll { overflow-y: auto; max-height: calc(100vh - 280px); padding-right: 4px; }
  .tab-scroll::-webkit-scrollbar { width: 4px; }
  .tab-scroll::-webkit-scrollbar-thumb { background: var(--glass-border); border-radius: 2px; }

  .browse-area { overflow-y: auto; scroll-behavior: smooth; max-height: calc(100vh - 280px); }
  .browse-area::-webkit-scrollbar { width: 4px; }
  .browse-area::-webkit-scrollbar-track { background: transparent; }
  .browse-area::-webkit-scrollbar-thumb { background: var(--glass-border); border-radius: 2px; }
  .browse-area::-webkit-scrollbar-thumb:hover { background: var(--text-muted); }

  .wp-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 14px; margin-bottom: 16px; }
  .wp-card { position: relative; border-radius: var(--r-md); overflow: hidden; aspect-ratio: 3 / 2; cursor: pointer; background: var(--glass-bg); border: 1px solid var(--glass-border); transition: transform 0.2s, box-shadow 0.2s; }
  .wp-card:hover { transform: translateY(-3px); box-shadow: 0 10px 30px rgba(0,0,0,0.4); border-color: var(--glass-border-hover); }
  .wp-card.skeleton { background: var(--glass-bg); overflow: hidden; position: relative; }
  .skeleton-pulse { position: absolute; inset: 0; background: linear-gradient(90deg,transparent 0%,rgba(255,255,255,0.04) 25%,rgba(255,255,255,0.08) 50%,rgba(255,255,255,0.04) 75%,transparent 100%); background-size: 200% 100%; animation: shimmer 1.8s ease-in-out infinite; }
  @keyframes shimmer { 0%{background-position:200% 0} 100%{background-position:-200% 0} }

  .fav-badge { position: absolute; top: 8px; right: 8px; z-index: 2; width: 22px; height: 22px; border-radius: 50%; background: rgba(255,107,107,0.85); color: #fff; font-size: 12px; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(255,107,107,0.4); pointer-events: none; }
  .wp-thumb { width: 100%; height: 100%; object-fit: cover; display: block; }
  .wp-thumb-placeholder { width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; font-size: 28px; font-weight: 700; color: rgba(255,255,255,0.15); }

  .wp-overlay { position: absolute; inset: 0; background: linear-gradient(to top,rgba(0,0,0,0.75) 0%,rgba(0,0,0,0.05) 55%,transparent 100%); opacity: 0; transition: opacity 0.25s ease; display: flex; flex-direction: column; justify-content: flex-end; padding: 12px; gap: 6px; }
  .wp-card:hover .wp-overlay { opacity: 1; }
  .wp-overlay .wp-actions { display: flex; gap: 6px; }
  .wp-overlay .wp-btn { width: 34px; height: 34px; border-radius: 50%; border: none; background: rgba(255,255,255,0.15); backdrop-filter: blur(8px); color: white; cursor: pointer; display: flex; align-items: center; justify-content: center; transition: all 0.2s; font-size: 14px; }
  .wp-overlay .wp-btn:hover { background: var(--accent); transform: scale(1.1); }
  .wp-overlay .wp-btn:disabled { opacity: 0.5; cursor: not-allowed; }
  .wp-overlay .wp-btn:disabled:hover { background: rgba(255,255,255,0.15); transform: none; }
  .wp-overlay .wp-btn.faved { color: #ff6b6b !important; }
  .wp-overlay .wp-source-tag { font-size: 10px; color: rgba(255,255,255,0.55); padding: 2px 8px; background: rgba(0,0,0,0.3); border-radius: 4px; align-self: flex-start; }

  /* Spinner inside button */
  .btn-spinner { width: 14px; height: 14px; border-radius: 50%; border: 2px solid rgba(255,255,255,0.3); border-top-color: #fff; animation: btnSpin 0.7s linear infinite; }
  @keyframes btnSpin { to { transform: rotate(360deg); } }

  .pagination-bar { display: flex; align-items: center; justify-content: center; gap: 6px; padding: 14px 0 20px; }
  .page-btn { min-width: 30px; height: 30px; border-radius: 8px; border: 1px solid var(--glass-border); background: var(--glass-bg); color: var(--text-secondary); font-size: 12px; cursor: pointer; display: flex; align-items: center; justify-content: center; padding: 0 8px; font-family: inherit; transition: all 0.15s; }
  .page-btn:hover { background: var(--glass-bg-hover); color: var(--text-primary); }
  .page-btn:disabled { opacity: 0.3; cursor: not-allowed; }
  .page-btn.active { background: var(--accent-soft); border-color: var(--accent); color: var(--accent); }
  .page-info { font-size: 11px; color: var(--text-muted); padding: 0 4px; }

  /* Toast notification — bottom-right */
  .toast {
    position: fixed; bottom: 28px; right: 28px; left: auto; transform: none; z-index: 999;
    padding: 12px 20px; border-radius: 10px;
    background: rgba(30, 39, 48, 0.92); backdrop-filter: blur(20px);
    border: 1px solid rgba(91,156,245,0.3);
    font-size: 13px; color: var(--text-primary);
    animation: toastSlideIn 0.35s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 8px 32px rgba(0,0,0,0.5);
    max-width: 320px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    pointer-events: none;
  }
  .toast.err { border-color: rgba(255,107,107,0.3); }
  @keyframes toastSlideIn {
    from { opacity: 0; transform: translateY(16px) scale(0.96); }
    to { opacity: 1; transform: translateY(0) scale(1); }
  }
</style>
