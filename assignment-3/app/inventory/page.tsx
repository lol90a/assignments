import { Shield } from "lucide-react";
import { DashboardCards } from "@/components/inventory/DashboardCards";
import { InventoryTable } from "@/components/inventory/InventoryTable";
import { listCertificates, summarizeInventory } from "@/lib/certificate-api";

type PageProps = {
  searchParams?: {
    query?: string;
    page?: string;
    expiresWithinDays?: string;
  };
};

export default async function InventoryPage({ searchParams }: Readonly<PageProps>) {
  // Search params are parsed on the server so the first render already contains
  // the filtered inventory and dashboard state.
  const page = Number(searchParams?.page ?? "1");
  const query = searchParams?.query ?? "";
  const expiresWithinDays = searchParams?.expiresWithinDays ? Number(searchParams.expiresWithinDays) : undefined;
  const inventory = await listCertificates({ query, expiresWithinDays, page, pageSize: 10 });
  // The dashboard intentionally uses a broad first page as a lightweight
  // substitute until the backend exposes a dedicated summary endpoint.
  const allVisibleForSummary = await listCertificates({ page: 1, pageSize: 500 });
  const summary = summarizeInventory(allVisibleForSummary.items);

  return (
    <main className="min-h-screen bg-surface">
      <div className="mx-auto flex max-w-7xl flex-col gap-6 px-4 py-6 sm:px-6 lg:px-8">
        <header className="flex flex-col gap-4 border-b border-line pb-6 md:flex-row md:items-center md:justify-between">
          <div>
            <div className="mb-2 inline-flex items-center gap-2 rounded-md border border-line bg-white px-3 py-1 text-sm font-medium text-accent">
              <Shield className="h-4 w-4" aria-hidden />
              Certificate Operations
            </div>
            <h1 className="text-3xl font-semibold tracking-normal text-ink">Inventory</h1>
            <p className="mt-2 max-w-3xl text-sm leading-6 text-slate-600">
              A server-rendered control surface for certificates issued by the Assignment 1 Rust backend.
            </p>
          </div>
          <a
            className="inline-flex h-10 items-center justify-center rounded-md bg-accent px-4 text-sm font-semibold text-white"
            href="/inventory?expiresWithinDays=30"
          >
            Expiring soon
          </a>
        </header>

        <DashboardCards summary={summary} />
        <InventoryTable initialData={inventory} query={query} expiresWithinDays={expiresWithinDays} />
      </div>
    </main>
  );
}
