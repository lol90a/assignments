import { clsx } from "clsx";

export function Card({ className, children }: Readonly<{ className?: string; children: React.ReactNode }>) {
  // A tiny shared wrapper keeps border, panel, and shadow styling consistent
  // without hiding the semantic content each caller renders inside.
  return <section className={clsx("rounded-lg border border-line bg-panel shadow-enterprise", className)}>{children}</section>;
}
