export function Skeleton({ label = "Loading" }: Readonly<{ label?: string }>) {
  // aria-label gives the loading placeholder a meaningful accessible name
  // without adding visible instructional copy to the UI.
  return (
    <div aria-label={label} className="animate-pulse rounded-md bg-slate-200">
      <div className="h-8" />
    </div>
  );
}
