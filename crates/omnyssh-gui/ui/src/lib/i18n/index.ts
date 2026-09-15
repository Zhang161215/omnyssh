import { derived, writable } from 'svelte/store';
import {
  DEFAULT_LOCALE,
  interpolate,
  isLocale,
  messages,
  type Locale,
  type Messages
} from './messages';

export type { Locale, Messages };
export { interpolate, isLocale, messages, DEFAULT_LOCALE };

const LOCAL_KEY = 'omnyssh-locale';
const STORE_FILE = 'settings.json';
const STORE_KEY = 'locale';

function mirrored(): Locale {
  try {
    const raw = localStorage.getItem(LOCAL_KEY);
    return isLocale(raw) ? raw : DEFAULT_LOCALE;
  } catch {
    return DEFAULT_LOCALE;
  }
}

function mirrorLocal(locale: Locale): void {
  try {
    localStorage.setItem(LOCAL_KEY, locale);
  } catch {
    // localStorage unavailable: the store copy is canonical.
  }
}

async function persistStore(locale: Locale): Promise<void> {
  try {
    const { load } = await import('@tauri-apps/plugin-store');
    const store = await load(STORE_FILE);
    await store.set(STORE_KEY, locale);
    await store.save();
  } catch {
    // Not under Tauri (tests, vite preview): the localStorage mirror suffices.
  }
}

function createLocale() {
  const initial = mirrored();
  const { subscribe, set: setStore } = writable<Locale>(initial);
  let current = initial;
  let interacted = false;

  function apply(next: Locale, user: boolean): void {
    current = next;
    setStore(next);
    mirrorLocal(next);
    if (user) {
      interacted = true;
      void persistStore(next);
    }
  }

  return {
    subscribe,
    set: (next: Locale) => apply(next, true),
    async hydrate(): Promise<void> {
      try {
        const { load } = await import('@tauri-apps/plugin-store');
        const store = await load(STORE_FILE);
        const saved = await store.get<string>(STORE_KEY);
        if (!interacted && isLocale(saved)) apply(saved, false);
      } catch {
        // Keep the mirrored value.
      }
    }
  };
}

export const locale = createLocale();

/** Reactive message tree for the active locale. */
export const t = derived(locale, ($locale): Messages => messages[$locale]);

export function formatError(code: string, copy: Messages): string {
  if (code.startsWith('portRange:')) {
    return interpolate(copy.errors.portRange, { value: code.slice('portRange:'.length) });
  }
  if (code.startsWith('probePortRange:')) {
    return interpolate(copy.errors.probePortRange, { value: code.slice('probePortRange:'.length) });
  }
  const table = copy.errors as Record<string, string>;
  return table[code] ?? code;
}
