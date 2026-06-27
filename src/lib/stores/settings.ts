import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface WallpaperItem {
  id: string;
  url: string;
  thumb: string;
  title: string;
  source: string;
  category: string;
  gradient: string;
  display_name?: string;
  attribution?: string;
  source_page?: string;
  created_at?: string;
}

export interface FavoriteItem {
  id: string;
  url: string;
  thumbnail: string;
  source: string;
  display_name: string;
  attribution: string;
  source_page: string;
  created_at: string;
}

export interface SourceStatus {
  name: string;
  display_name: string;
  enabled: boolean;
  has_api_key: boolean;
  needs_api_key: boolean;
  hot_categories: string[];
}

export interface ScheduleConfig {
  enabled: boolean;
  hour: number;
  minute: number;
  set_desktop: boolean;
  set_lock_screen: boolean;
  frequency: string;
}

export interface WallpaperInfo {
  url: string;
  path: string;
  source: string;
  lastUpdate: string;
}

export const sources = writable<SourceStatus[]>([]);
export const schedule = writable<ScheduleConfig>({
  enabled: true,
  hour: 10,
  minute: 0,
  set_desktop: true,
  set_lock_screen: false,
  frequency: 'daily',
});
export const categories = writable<string[]>(['自然风景', '极简', '星空', '城市夜景']);
export const wallpaperInfo = writable<WallpaperInfo>({ url: '', path: '', source: '', lastUpdate: '' });
export const activeTab = writable<string>('sources');

// Browse state
export const selectedSource = writable<string>('');
export const selectedCategory = writable<string>('all');
export const browsePage = writable<number>(1);

export interface WallpaperItem {
  id: string;
  url: string;
  thumb: string;
  title: string;
  source: string;
  category: string;
  gradient: string;
}

export async function loadSettings() {
  try {
    const result: any = await invoke('get_settings');
    sources.set(Object.entries(result.sources).map(([name, status]: any) => ({
      name,
      display_name: status.display_name,
      enabled: status.enabled,
      has_api_key: status.has_api_key,
      needs_api_key: ['pexels', 'unsplash'].includes(name),
      hot_categories: [],
    })));
    schedule.set(result.schedule);
    categories.set(result.categories);
    wallpaperInfo.set(result.wallpaper);
  } catch (e) {
    console.warn('Failed to load settings:', e);
  }
}

export async function loadSourcesStatus() {
  try {
    const result: SourceStatus[] = await invoke('get_sources_status');
    sources.set(result);
  } catch (e) {
    console.warn('Failed to load sources:', e);
  }
}

export async function toggleSource(name: string, enabled: boolean) {
  try {
    await invoke('set_source_enabled', { source: name, enabled });
    await loadSourcesStatus();
  } catch (e) {
    console.error('Failed to toggle source:', e);
  }
}

export async function setApiKey(source: string, apiKey: string) {
  try {
    await invoke('set_api_key', { source, apiKey });
  } catch (e) {
    console.error('Failed to set API key:', e);
  }
}

export async function addCategory(category: string) {
  try {
    const result: string[] = await invoke('add_user_category', { category });
    categories.set(result);
  } catch (e) {
    console.error('Failed to add category:', e);
  }
}

export async function removeCategory(category: string) {
  try {
    const result: string[] = await invoke('remove_user_category', { category });
    categories.set(result);
  } catch (e) {
    console.error('Failed to remove category:', e);
  }
}

export async function updateSchedule(s: ScheduleConfig) {
  try {
    await invoke('set_schedule', { schedule: s });
    schedule.set(s);
  } catch (e) {
    console.error('Failed to update schedule:', e);
  }
}

export async function nextWallpaper() {
  try {
    await invoke('next_wallpaper');
  } catch (e) {
    console.error('Failed to change wallpaper:', e);
  }
}

export async function saveWallpaper() {
  try {
    const result: string = await invoke('save_current_wallpaper');
    return result;
  } catch (e) {
    console.error('Failed to save wallpaper:', e);
    return null;
  }
}

export async function browseSource(source: string, category: string, page: number): Promise<WallpaperItem[]> {
  try {
    const result: any[] = await invoke('browse_source', { source, category, page });
    return result.map((item: any) => ({
      id: item.id,
      url: item.url,
      thumb: item.thumbnail,
      title: item.attribution,
      source: item.source,
      display_name: item.display_name,
      category: category,
      gradient: '',
      source_page: item.source_page,
    }));
  } catch (e) {
    console.warn('Browse failed:', e);
    return [];
  }
}

export async function setWallpaperFrom(url: string, source: string): Promise<boolean> {
  try {
    await invoke('set_wallpaper_from', { url, source });
    return true;
  } catch (e) {
    console.error('Failed to set wallpaper:', e);
    return false;
  }
}

// === Favorites ===
export const favorites = writable<FavoriteItem[]>([]);

export async function loadFavorites() {
  try {
    const result: FavoriteItem[] = await invoke('get_favorites');
    favorites.set(result);
  } catch (e) {
    console.warn('Failed to load favorites:', e);
  }
}

export async function addFavorite(item: {
  id: string; url: string; thumbnail: string;
  source: string; display_name: string; attribution: string; source_page: string;
}) {
  try {
    const result: FavoriteItem[] = await invoke('add_favorite', { item });
    favorites.set(result);
    return true;
  } catch (e) {
    console.warn('Add favorite error:', e);
    return false;
  }
}

export async function removeFavorite(id: string) {
  try {
    const result: FavoriteItem[] = await invoke('remove_favorite', { id });
    favorites.set(result);
    return true;
  } catch (e) {
    console.error('Remove favorite error:', e);
    return false;
  }
}

export async function isFavorited(id: string): Promise<boolean> {
  try {
    return await invoke('is_favorited', { id });
  } catch { return false; }
}

// === Auto-start ===
export async function setAutoStart(enabled: boolean): Promise<boolean> {
  try {
    await invoke('set_auto_start', { enabled });
    return true;
  } catch (e) {
    console.error('Auto-start toggle failed:', e);
    return false;
  }
}

export async function getAutoStart(): Promise<boolean> {
  try {
    return await invoke('get_auto_start');
  } catch { return false; }
}
