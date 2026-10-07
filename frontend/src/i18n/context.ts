import { createContext, useContext } from 'react';
import type { Dict } from './zh-CN';

export interface I18nContextValue {
  t: (key: keyof Dict) => string;
}

export const I18nContext = createContext<I18nContextValue>({
  t: (key) => String(key),
});

export function useI18n() {
  return useContext(I18nContext);
}
