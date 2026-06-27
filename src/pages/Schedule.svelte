<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../lib/i18n';
  import { schedule, updateSchedule, nextWallpaper, setAutoStart, getAutoStart } from '../lib/stores/settings';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  let sch = $state({ ...$schedule });
  let autoStart = $state(false);
  let dwPath = $state('');
  let openFolder = $state(true);
  // Update state
  let updateInfo = $state<{ has_update: boolean; latest_version: string; download_url: string; release_notes: string } | null>(null);
  let checkingUpdate = $state(false);
  let downloadingUpdate = $state(false);

  onMount(async () => {
    autoStart = await getAutoStart();
    try {
      const ds: any = await invoke('get_download_settings');
      dwPath = ds.download_path;
      openFolder = ds.open_folder_after_download;
    } catch {}
    // Auto-check for update
    checkForUpdate();
  });

  async function pickDownloadPath() {
    try {
      const selected = await open({ directory: true, multiple: false, title: '选择壁纸保存目录' });
      if (selected && typeof selected === 'string') {
        dwPath = selected;
        await invoke('set_download_path', { path: selected });
      }
    } catch (e) { console.warn('Folder picker:', e); }
  }

  async function checkForUpdate() {
    checkingUpdate = true;
    try {
      const info: any = await invoke('check_update');
      updateInfo = info;
    } catch {}
    checkingUpdate = false;
  }

  async function doUpdate() {
    if (!updateInfo?.has_update || !updateInfo?.download_url) return;
    downloadingUpdate = true;
    try {
      const scriptPath: string = await invoke('download_update', { url: updateInfo.download_url });
      await invoke('apply_update', { scriptPath });
    } catch (e) { console.error('Update failed:', e); }
    downloadingUpdate = false;
  }

  $effect(() => {
    if ($schedule) {
      sch = { ...$schedule };
    }
  });

  function getTimeValue(): number {
    return sch.hour * 60 + sch.minute;
  }

  function setTimeFromValue(val: number) {
    sch.hour = Math.floor(val / 60);
    sch.minute = val % 60;
  }

  function handleSlider(e: Event) {
    const target = e.target as HTMLInputElement;
    setTimeFromValue(parseInt(target.value));
  }

  function formatTime(): string {
    const h = sch.hour;
    const m = sch.minute.toString().padStart(2, '0');
    if (h >= 12) {
      return `${h === 12 ? 12 : h - 12}:${m} PM`;
    }
    return `${h === 0 ? 12 : h}:${m} AM`;
  }

  function handleSave() {
    updateSchedule(sch);
  }

  function handleNow() {
    nextWallpaper();
  }

  const frequencies = [
    { value: 'daily', label: $t('schedule.frequency_daily') },
    { value: '12h', label: $t('schedule.frequency_12h') },
    { value: '6h', label: $t('schedule.frequency_6h') },
    { value: '1h', label: $t('schedule.frequency_1h') },
  ];

  let isMac = $state(false);
  try {
    if (navigator.platform?.toLowerCase().includes('mac')) isMac = true;
  } catch {}
</script>

<div class="glass-card">
  <div class="section-header">
    <span class="section-title">{$t('schedule.title')}</span>
    <div class="header-actions">
      <button class="now-btn" onclick={handleNow}>🔄 Update Now</button>
    </div>
  </div>

  <div class="schedule-main">
    <div class="time-picker-col">
      <div class="time-display">
        <span class="time-value">{formatTime()}</span>
      </div>
      <input
        type="range"
        class="time-slider"
        min="0"
        max="1440"
        value={getTimeValue()}
        oninput={handleSlider}
        onchange={handleSave}
      />
      <div class="time-labels">
        <span>00:00</span>
        <span>12:00</span>
        <span>23:59</span>
      </div>
    </div>

    <div class="schedule-options">
      <div class="option-row">
        <div>
          <div class="option-label">🖥 {$t('schedule.desktop_label')}</div>
          <div class="option-desc">{$t('schedule.desktop_desc')}</div>
        </div>
        <button
          class="checkbox"
          class:checked={sch.set_desktop}
          onclick={() => { sch.set_desktop = !sch.set_desktop; handleSave(); }}
          aria-label="Toggle desktop wallpaper"
        ></button>
      </div>

      {#if !isMac}
        <div class="option-row">
          <div>
            <div class="option-label">🔒 {$t('schedule.lock_label')}</div>
            <div class="option-desc">{$t('schedule.lock_desc')}</div>
          </div>
          <button
            class="checkbox"
            class:checked={sch.set_lock_screen}
            onclick={() => { sch.set_lock_screen = !sch.set_lock_screen; handleSave(); }}
            aria-label="Toggle lock screen wallpaper"
          ></button>
        </div>
      {/if}

      <div class="option-row">
        <div>
          <div class="option-label">🚀 {$t('schedule.autostart_label')}</div>
          <div class="option-desc">{$t('schedule.autostart_desc')}</div>
        </div>
        <button
          class="checkbox"
          class:checked={autoStart}
          onclick={async () => { autoStart = await setAutoStart(!autoStart); }}
          aria-label="Toggle auto-start"
        ></button>
      </div>

      <div class="option-row">
        <div>
          <div class="option-label">📅 {$t('schedule.frequency_label')}</div>
        </div>
        <select
          class="freq-select"
          value={sch.frequency}
          onchange={(e) => { sch.frequency = (e.target as HTMLSelectElement).value; handleSave(); }}
        >
          {#each frequencies as f}
            <option value={f.value}>{f.label}</option>
          {/each}
        </select>
      </div>
    </div>
  </div>

  {#if isMac}
    <div class="mac-hint">
      <strong>💡 {$t('schedule.mac_hint')}</strong>
    </div>
  {/if}
</div>

<!-- Download Settings -->
<div class="glass-card" style="margin-top:20px;">
  <div class="section-header">
    <span class="section-title">📥 下载设置</span>
  </div>
  <div class="schedule-options">
    <div class="option-row">
      <div>
        <div class="option-label">📂 保存路径</div>
        <div class="option-desc" style="font-size:11px;word-break:break-all;">{dwPath || 'Pictures/Wallery（默认）'}</div>
      </div>
      <button class="btn-change" onclick={pickDownloadPath}>更改</button>
    </div>
    <div class="option-row">
      <div>
        <div class="option-label">📂 下载后打开文件夹</div>
        <div class="option-desc">下载完成后自动打开资源管理器</div>
      </div>
      <button
        class="checkbox"
        class:checked={openFolder}
        onclick={async () => {
          openFolder = !openFolder;
          try { await invoke('set_open_folder_after_download', { enabled: openFolder }); } catch {}
        }}
        aria-label="Toggle open folder after download"
      ></button>
    </div>
  </div>
</div>

<!-- Update Section -->
<div class="glass-card" style="margin-top:20px;">
  <div class="section-header">
    <span class="section-title">🔄 软件更新</span>
  </div>
  <div class="schedule-options">
    {#if checkingUpdate}
      <div style="font-size:12px;color:var(--text-muted);">正在检查更新…</div>
    {:else if updateInfo?.has_update}
      <div class="option-row">
        <div>
          <div class="option-label">发现新版本 v{updateInfo.latest_version}</div>
          <div class="option-desc" style="font-size:11px;max-height:60px;overflow:hidden;">{updateInfo.release_notes}</div>
        </div>
        <button class="btn-change" disabled={downloadingUpdate} onclick={doUpdate}>
          {downloadingUpdate ? '下载中…' : '立即更新'}
        </button>
      </div>
    {:else}
      <div style="font-size:12px;color:var(--text-muted);">已是最新版本 v1.0.0</div>
    {/if}
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
  .header-actions {
    display: flex;
    gap: 8px;
  }
  .now-btn {
    padding: 6px 14px;
    border-radius: 100px;
    border: 1px solid var(--accent, #5b9cf5);
    background: var(--accent-soft, rgba(91,156,245,0.15));
    color: var(--accent, #5b9cf5);
    font-size: 12px;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.3s;
  }
  .now-btn:hover {
    background: var(--accent, #5b9cf5);
    color: white;
  }
  .schedule-main {
    display: flex;
    align-items: flex-start;
    gap: 32px;
    flex-wrap: wrap;
  }
  .time-picker-col { flex: 0 0 auto; }
  .time-value {
    font-size: 56px;
    font-weight: 300;
    letter-spacing: -2px;
    color: var(--text-primary, #e8eaed);
    line-height: 1;
    font-feature-settings: 'tnum';
  }
  .time-slider {
    width: 260px;
    -webkit-appearance: none;
    appearance: none;
    height: 4px;
    border-radius: 2px;
    background: rgba(255,255,255,0.08);
    outline: none;
    margin-top: 8px;
    cursor: pointer;
  }
  .time-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: linear-gradient(145deg, #fff, #b0c4e0);
    border: none;
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(0,0,0,0.3), 0 0 12px var(--accent-glow, rgba(91,156,245,0.25));
  }
  .time-labels {
    display: flex;
    justify-content: space-between;
    margin-top: 6px;
  }
  .time-labels span { font-size: 10px; color: var(--text-muted, #5f6672); }
  .schedule-options { flex: 1; min-width: 280px; }
  .option-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 0;
    border-bottom: 1px solid rgba(255,255,255,0.04);
    gap: 12px;
  }
  .option-row:last-child { border-bottom: none; }
  .btn-change {
    padding: 5px 14px;
    border-radius: 100px;
    border: 1px solid var(--glass-border);
    background: var(--glass-bg);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    flex-shrink: 0;
    transition: all 0.2s;
  }
  .btn-change:hover { background: var(--accent); color: white; border-color: var(--accent); }
  .btn-change:disabled { opacity: 0.5; cursor: not-allowed; }
  .option-label {
    font-size: 13.5px;
    color: var(--text-primary, #e8eaed);
    font-weight: 400;
    white-space: nowrap;
  }
  .option-desc {
    font-size: 11.5px;
    color: var(--text-muted, #5f6672);
    margin-top: 2px;
  }
  .checkbox {
    width: 20px; height: 20px; border-radius: 6px;
    border: 1.5px solid rgba(255,255,255,0.15);
    background: rgba(0,0,0,0.1);
    cursor: pointer; flex-shrink: 0;
    display: flex; align-items: center; justify-content: center;
    transition: all 0.3s; padding: 0;
  }
  .checkbox.checked {
    background: var(--accent, #5b9cf5);
    border-color: var(--accent, #5b9cf5);
  }
  .checkbox.checked::after {
    content: '✓'; color: white; font-size: 12px; font-weight: 700;
  }
  .freq-select {
    padding: 6px 12px; border-radius: 8px;
    border: 1px solid rgba(255,255,255,0.1);
    background: rgba(0,0,0,0.2);
    color: var(--text-primary, #e8eaed);
    font-size: 12.5px; font-family: inherit;
    outline: none; cursor: pointer;
  }
  .mac-hint {
    margin-top: 16px; padding: 12px 16px; border-radius: 12px;
    background: rgba(91, 156, 245, 0.06);
    border: 1px solid rgba(91, 156, 245, 0.12);
    font-size: 12px; color: var(--text-secondary, #9aa0ab); line-height: 1.6;
  }
  .mac-hint strong { color: var(--accent, #5b9cf5); }
</style>
