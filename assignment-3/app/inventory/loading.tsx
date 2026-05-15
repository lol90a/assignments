import { Skeleton } from "@/components/ui/Skeleton";

export default function InventoryLoading() {
  // Route-level loading mirrors the page structure so layout shift is small
  // while server components fetch inventory data.
  return (
    <main className="mx-auto flex max-w-7xl flex-col gap-4 px-4 py-6">
      <Skeleton label="Loading dashboard" />
      <Skeleton label="Loading inventory" />
      <Skeleton label="Loading table" />
    </main>
  );
}
