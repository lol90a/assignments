// Mirrors the JSON shape returned by Assignment 1's Rust certificate API.
export type Certificate = {
  certificate_id: string;
  subject: string;
  issuer: string;
  expiration: string;
  san_entries: string[];
};

// Filters are kept serializable so they can flow through URL search params and
// the Next.js API route without custom parsing.
export type InventoryFilters = {
  query?: string;
  expiresWithinDays?: number;
  page: number;
  pageSize: number;
};

// Paginated inventory response consumed by both SSR pages and SWR refreshes.
export type InventoryResult = {
  items: Certificate[];
  total: number;
  page: number;
  pageSize: number;
};

// Aggregate counts shown in the dashboard cards.
export type DashboardSummary = {
  total: number;
  expiringSoon: number;
  uniqueIssuers: number;
  expired: number;
};
