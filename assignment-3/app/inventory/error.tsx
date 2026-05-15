"use client";

// Next.js error boundaries must be client components because the reset callback
// is interactive. The rest of the page remains server-rendered for secure data
// access.
export default function InventoryError({ reset }: Readonly<{ reset: () => void }>) {
  // Next.js passes reset so the user can retry the failed server render without
  // leaving the inventory route.
  return (
    <main className="flex min-h-screen items-center justify-center bg-surface p-6">
      <section className="max-w-lg rounded-lg border border-line bg-white p-6 shadow-enterprise">
        <h1 className="text-xl font-semibold text-ink">Inventory unavailable</h1>
        <p className="mt-2 text-sm leading-6 text-slate-600">
          The frontend could not reach the Assignment 1 Rust backend. Verify `ASSIGNMENT1_API_BASE_URL`, TLS settings, and the
          certificate IDs configured for SSR hydration.
        </p>
        <button className="mt-4 rounded-md bg-ink px-4 py-2 text-sm font-medium text-white" onClick={reset}>
          Retry
        </button>
      </section>
    </main>
  );
}
