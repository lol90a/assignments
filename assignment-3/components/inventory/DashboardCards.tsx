import { AlertTriangle, BadgeCheck, Database, ShieldCheck } from "lucide-react";
import { Card } from "../ui/Card";
import type { DashboardSummary } from "@/lib/types";

const cards = [
  { key: "total", label: "Total certificates", icon: Database },
  { key: "expiringSoon", label: "Expiring in 30 days", icon: AlertTriangle },
  { key: "uniqueIssuers", label: "Issuers", icon: ShieldCheck },
  { key: "expired", label: "Expired", icon: BadgeCheck }
] as const;

export function DashboardCards({ summary }: Readonly<{ summary: DashboardSummary }>) {
  // Card definitions above make the layout data-driven without duplicating the
  // same markup four times.
  return (
    <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {cards.map((card) => {
        const Icon = card.icon;
        return (
          <Card key={card.key} className="p-4">
            <div className="flex items-center justify-between gap-3">
              <div>
                <p className="text-sm font-medium text-slate-500">{card.label}</p>
                <p className="mt-2 text-3xl font-semibold text-ink">{summary[card.key]}</p>
              </div>
              <div className="rounded-md border border-line bg-surface p-2 text-accent">
                <Icon className="h-5 w-5" aria-hidden />
              </div>
            </div>
          </Card>
        );
      })}
    </div>
  );
}
