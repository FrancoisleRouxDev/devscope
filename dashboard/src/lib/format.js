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
  if (!unixSeconds) return 'unknown';
  const days = Math.floor((Date.now() / 1000 - unixSeconds) / 86400);
  if (days < 1) return 'today';
  if (days < 31) return `${days} day${days === 1 ? '' : 's'} ago`;
  if (days < 365) return `${Math.floor(days / 30)} mo ago`;
  return `${Math.floor(days / 365)} yr ago`;
}
