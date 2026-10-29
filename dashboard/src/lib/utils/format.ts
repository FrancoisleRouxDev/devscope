// =============================================================================
// src/lib/utils/format.ts — pure display formatters
// =============================================================================
// Small, pure, unit-testable helpers. Uses `Intl` so numbers follow the user's
// OS locale (1,234,567 in en-US; 1 234 567 in fr-FR) on every platform.
// =============================================================================

// =============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// =============================================================================
//
// const int = new Intl.NumberFormat(undefined);
// const compact = new Intl.NumberFormat(undefined, { notation: 'compact', maximumFractionDigits: 1 });
//
// export const formatInt = (n: number) => int.format(n);          // 1234567 → "1,234,567"
// export const formatCompact = (n: number) => compact.format(n);  // 1234567 → "1.2M"
//
// export function formatBytes(bytes: number): string { /* 1536 → "1.5 KB" */ }
// export function formatDuration(ms: number): string { /* 1830 → "1.8 s" */ }
// export function percent(part: number, whole: number): string { /* guard whole === 0 */ }

export {};
