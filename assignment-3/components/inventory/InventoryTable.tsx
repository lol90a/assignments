"use client";

import Link from "next/link";
import useSWR from "swr";
import { ChevronLeft, ChevronRight, ExternalLink, Search } from "lucide-react";
import { Badge } from "../ui/Badge";
import { Card } from "../ui/Card";
import { Skeleton } from "../ui/Skeleton";
import { certificateState, daysUntil, formatDate } from "@/lib/format";
import type { InventoryResult } from "@/lib/types";

// The table is the only client-heavy part of the inventory page. It uses SWR
// for freshness after the SSR payload arrives, while filtering and pagination
// still round-trip through server routes so backend details stay hidden.
type Props = {
  initialData: InventoryResult;
  query: string;
  expiresWithinDays?: number;
};

const fetcher = async (url: string): Promise<InventoryResult> => {
  const response = await fetch(url);
  if (!response.ok) throw new Error("Unable to load certificate inventory");
  return (await response.json()) as InventoryResult;
};

export function InventoryTable({ initialData, query, expiresWithinDays }: Readonly<Props>) {
  // Build a stable query string from the server-rendered filters so SWR asks
  // for the same slice of inventory the page initially displayed.
  const params = new URLSearchParams({
    page: String(initialData.page),
    pageSize: String(initialData.pageSize)
  });
  if (query) params.set("query", query);
  if (expiresWithinDays) params.set("expiresWithinDays", String(expiresWithinDays));

  // SWR is used after SSR so navigation and refreshes feel fast without losing
  // the SEO/security benefits of server-rendered initial data.
  const { data, error, isLoading } = useSWR(`/inventory/api?${params.toString()}`, fetcher, {
    fallbackData: initialData,
    revalidateOnFocus: true
  });

  const inventory = data ?? initialData;
  const showLoading = isLoading && !data && initialData.items.length === 0;
  const pageCount = Math.max(1, Math.ceil(inventory.total / inventory.pageSize));

  return (
    <Card className="overflow-hidden">
      <div className="flex flex-col gap-3 border-b border-line p-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h2 className="text-lg font-semibold text-ink">Certificate inventory</h2>
          <p className="text-sm text-slate-500">Server-rendered from the Rust backend, refreshed in-place with SWR.</p>
        </div>
        <form className="flex w-full gap-2 md:w-auto" action="/inventory">
          <label className="relative flex-1 md:w-80">
            <Search className="pointer-events-none absolute left-3 top-2.5 h-4 w-4 text-slate-400" aria-hidden />
            <input
              name="query"
              defaultValue={query}
              placeholder="Subject, issuer, or SAN"
              className="h-9 w-full rounded-md border border-line bg-white pl-9 pr-3 text-sm outline-none focus:border-accent focus:ring-2 focus:ring-teal-100"
            />
          </label>
          <button className="h-9 rounded-md bg-ink px-4 text-sm font-medium text-white" type="submit">
            Filter
          </button>
        </form>
      </div>

      {showLoading ? (
        <div className="space-y-3 p-4">
          <Skeleton />
          <Skeleton />
          <Skeleton />
        </div>
      ) : error ? (
        <div className="p-6 text-sm text-danger">The inventory could not be loaded. Check the backend URL and TLS settings.</div>
      ) : inventory.items.length === 0 ? (
        <div className="p-8 text-center text-sm text-slate-500">
          No certificates matched the current filters. Create records in Assignment 1 and refresh this page.
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full border-collapse text-left text-sm">
            <thead className="bg-surface text-xs uppercase tracking-wide text-slate-500">
              <tr>
                <th className="px-4 py-3">Subject</th>
                <th className="px-4 py-3">Issuer</th>
                <th className="px-4 py-3">SANs</th>
                <th className="px-4 py-3">Expiration</th>
                <th className="px-4 py-3">State</th>
                <th className="px-4 py-3" aria-label="Actions" />
              </tr>
            </thead>
            <tbody className="divide-y divide-line">
              {inventory.items.map((certificate) => {
                const state = certificateState(certificate.expiration);
                // Keep tone selection local to row rendering because it is a
                // presentation concern, not part of the backend certificate DTO.
                const tone = state === "Expired" ? "danger" : state === "Expiring soon" ? "warning" : "success";
                return (
                  <tr key={certificate.certificate_id} className="bg-white">
                    <td className="max-w-xs px-4 py-3 font-medium text-ink">{certificate.subject}</td>
                    <td className="px-4 py-3 text-slate-600">{certificate.issuer}</td>
                    <td className="px-4 py-3 text-slate-600">{certificate.san_entries.slice(0, 2).join(", ")}</td>
                    <td className="px-4 py-3 text-slate-600">
                      {formatDate(certificate.expiration)}
                      <span className="block text-xs text-slate-400">{daysUntil(certificate.expiration)} days</span>
                    </td>
                    <td className="px-4 py-3">
                      <Badge tone={tone}>{state}</Badge>
                    </td>
                    <td className="px-4 py-3 text-right">
                      <Link className="inline-flex items-center gap-1 text-sm font-medium text-accent" href={`/inventory/${certificate.certificate_id}`}>
                        Details <ExternalLink className="h-3.5 w-3.5" aria-hidden />
                      </Link>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}

      <div className="flex items-center justify-between border-t border-line p-4 text-sm text-slate-500">
        <span>
          Page {inventory.page} of {pageCount}
        </span>
        <div className="flex gap-2">
          <Link
            className="inline-flex h-9 items-center gap-1 rounded-md border border-line px-3 text-slate-700"
            href={`/inventory?page=${Math.max(1, inventory.page - 1)}${query ? `&query=${encodeURIComponent(query)}` : ""}`}
          >
            <ChevronLeft className="h-4 w-4" aria-hidden /> Previous
          </Link>
          <Link
            className="inline-flex h-9 items-center gap-1 rounded-md border border-line px-3 text-slate-700"
            href={`/inventory?page=${Math.min(pageCount, inventory.page + 1)}${query ? `&query=${encodeURIComponent(query)}` : ""}`}
          >
            Next <ChevronRight className="h-4 w-4" aria-hidden />
          </Link>
        </div>
      </div>
    </Card>
  );
}
