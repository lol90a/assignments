import { differenceInCalendarDays, format, parseISO } from "date-fns";

// Date and status formatting lives outside components so table rows, dashboard
// cards, and detail pages cannot drift into subtly different interpretations of
// expiration state.
export function formatDate(value: string): string {
  // Backend timestamps are ISO strings; parsing at the boundary keeps display
  // formatting consistent across tables, cards, and detail views.
  return format(parseISO(value), "MMM d, yyyy HH:mm 'UTC'");
}

export function daysUntil(value: string): number {
  return differenceInCalendarDays(parseISO(value), new Date());
}

export function certificateState(expiration: string): "Expired" | "Expiring soon" | "Healthy" {
  // The same state logic drives both badge text and badge color selection.
  const days = daysUntil(expiration);
  if (days < 0) return "Expired";
  if (days <= 30) return "Expiring soon";
  return "Healthy";
}
