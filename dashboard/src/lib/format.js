// format.js - small display helpers shared by components.

// 1536 -> "1.5 KB". Uses 1024-based units like most file managers.
export function formatBytes(n) {
  if (!n) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.min(Math.floor(Math.log(n) / Math.log(1024)), units.length - 1);
  const value = n / 1024 ** i;
  return `${value >= 10 || i === 0 ? Math.round(value) : value.toFixed(1)} ${units[i]}`;
}

// Unix seconds -> "3 days ago". 0 means unknown.
export function timeAgo(unixSeconds) {
  // Some tools stamp files with 1970 (the epoch); that is a missing date, not a 46-year-old file.
  if (!unixSeconds || unixSeconds < 31_536_000) return 'unknown';
  const days = Math.floor((Date.now() / 1000 - unixSeconds) / 86400);
  if (days < 1) return 'today';
  if (days < 31) return `${days} day${days === 1 ? '' : 's'} ago`;
  if (days < 365) return `${Math.floor(days / 30)} mo ago`;
  return `${Math.floor(days / 365)} yr ago`;
}

// "C:\Users\me\Downloads\setup.exe" -> "setup.exe" (handles both separators).
export function baseName(path) {
  return path.split(/[\\/]/).pop();
}

// "C:\Users\me\Downloads\setup.exe" -> "C:\Users\me\Downloads"
export function dirName(path) {
  const i = Math.max(path.lastIndexOf('\\'), path.lastIndexOf('/'));
  return i > 0 ? path.slice(0, i) : path;
}

// One fixed colour per file category so bars and legends always match.
const CATEGORY_COLORS = {
  Video: 'var(--series-2)',
  Images: 'var(--series-3)',
  Audio: 'var(--series-5)',
  Documents: 'var(--series-1)',
  Archives: 'var(--series-4)',
  Installers: 'var(--danger)',
  Code: 'var(--series-6)',
  Other: 'var(--series-7)',
};
export const categoryColor = (name) => CATEGORY_COLORS[name] ?? 'var(--series-7)';
