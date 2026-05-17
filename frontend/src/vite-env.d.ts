/// <reference types="vite/client" />

// Variáveis de ambiente customizadas da aplicação
interface ImportMetaEnv {
  /** URL base da API backend (ex: http://localhost:3000) */
  readonly VITE_API_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
