import { useCallback, type ReactNode } from 'react';
import { zhCN, type Dict } from './zh-CN';
import { I18nContext, type I18nContextValue } from './context';

export function LanguageProvider({ children }: { children: ReactNode }) {
  const t = useCallback((key: keyof Dict) => {
    return zhCN[key] ?? String(key);
  }, []);

  const value: I18nContextValue = { t };

  return (
    <I18nContext.Provider value={value}>
      {children}
    </I18nContext.Provider>
  );
}
