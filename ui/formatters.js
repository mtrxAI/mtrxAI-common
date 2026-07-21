/**
 * Shared GPU / location formatters for peer and server dashboards.
 * Copy into Alpine component methods, or sync when editing UI HTML.
 */

export function fmtGpu(g) {
  if (!g?.available) return '';
  const parts = [];
  if (g.name) parts.push(g.name);
  if (g.producer && g.architecture) parts.push(`${g.producer} ${g.architecture}`);
  else if (g.producer) parts.push(g.producer);
  const stats = `${g.utilization_pct ?? 0}% util · ${g.memory_used_mb ?? 0}/${g.memory_total_mb ?? 0} MB`;
  if (g.temperature_c != null) return `${parts.join(' · ') || 'GPU'} · ${stats} · ${g.temperature_c}°C`;
  return `${parts.join(' · ') || 'GPU'} · ${stats}`;
}

/** Server dashboard style (shorter). */
export function fmtGpuShort(g) {
  if (!g || !g.available) return '—';
  return `${g.name || 'GPU'} · ${g.utilization_pct}% · ${g.memory_used_mb}/${g.memory_total_mb} MB`;
}

export function locationLine(peer) {
  const loc = peer?.location;
  if (loc?.lat == null || loc?.lon == null) return 'Location unknown';
  const place = [loc.city, loc.country].filter(Boolean).join(', ') || peer.id || peer.peer_id || '—';
  return `${loc.lat.toFixed(4)}, ${loc.lon.toFixed(4)} · ${place}`;
}
