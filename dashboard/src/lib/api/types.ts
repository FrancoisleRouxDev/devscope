// =============================================================================
// src/lib/api/types.ts — TypeScript mirror of the API's JSON contract
// =============================================================================
// Mirrors api/devscope_api/schemas.py (camelCase, as the API serialises it),
// which in turn mirrors scanner/src/stats.rs.
//
// Two ways to keep these in sync:
//   * by hand (fine while the API is small), or
//   * generated: `npx openapi-typescript http://127.0.0.1:<port>/openapi.json -o src/lib/api/schema.d.ts`
//     — preferred once the API grows; then this file just re-exports from it.
// =============================================================================

// =============================================================================
// PLANNED TYPES (commented out — not implemented yet)
// =============================================================================
//
// export interface LineCounts { code: number; comments: number; blanks: number }
//
// export interface LanguageStats { language: string; files: number; lines: LineCounts; bytes: number }
//
// export interface FileStats { path: string; language: string; lines: LineCounts; bytes: number }
//
// export interface ScanReport {
//   root: string;
//   totalFiles: number;
//   totalLines: number;
//   languages: LanguageStats[];
//   largestFiles: FileStats[];
//   skipped: number;
//   durationMs: number;
// }
//
// export type JobStatus = 'queued' | 'running' | 'done' | 'failed';
//
// export interface ScanRequest {
//   path: string;
//   respectGitignore?: boolean;
//   includeHidden?: boolean;
//   maxFileSize?: number;
//   topNFiles?: number;
// }
//
// export interface ScanJob {
//   id: string;
//   status: JobStatus;
//   request: ScanRequest;
//   createdAt: string;           // ISO 8601
//   report: ScanReport | null;
//   error: string | null;
// }

export {};
