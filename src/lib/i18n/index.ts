import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Static translation data
import enData from './en.json';
import zhData from './zh.json';

// Language detection
function detectLanguage(): string {
  const lang = navigator.language || (navigator as any).userLanguage || 'en';
  if (lang.startsWith('zh')) return 'zh';
  return 'en';
}

export const currentLang = writable<string>(detectLanguage());

const translations: Record<string, any> = { en: enData, zh: zhData };

export const t = derived(currentLang, ($lang) => {
  const dict = translations[$lang] || translations['en'];

  return (key: string): string => {
    const keys = key.split('.');
    let value: any = dict;
    for (const k of keys) {
      if (value && typeof value === 'object' && k in value) {
        value = value[k];
      } else {
        return key;
      }
    }
    return typeof value === 'string' ? value : key;
  };
});

export function setLanguage(lang: string) {
  if (lang === 'auto') {
    currentLang.set(detectLanguage());
  } else {
    currentLang.set(lang);
  }
}

// Listen for Tauri events to switch language
listen('language-changed', (event: any) => {
  setLanguage(event.payload as string);
});
