import { clsx } from "clsx";

// Badge tones are intentionally limited. A small status vocabulary keeps the
// certificate table scannable and avoids one-off colors that weaken visual
// consistency.
type Tone = "neutral" | "success" | "warning" | "danger";

const tones: Record<Tone, string> = {
  neutral: "border-slate-200 bg-slate-50 text-slate-700",
  success: "border-emerald-200 bg-emerald-50 text-emerald-700",
  warning: "border-amber-200 bg-amber-50 text-amber-700",
  danger: "border-red-200 bg-red-50 text-red-700"
};

export function Badge({ tone = "neutral", children }: Readonly<{ tone?: Tone; children: React.ReactNode }>) {
  // Centralizing tone classes keeps status colors consistent across inventory
  // table rows, detail pages, and summary surfaces.
  return <span className={clsx("inline-flex rounded-full border px-2 py-0.5 text-xs font-medium", tones[tone])}>{children}</span>;
}
