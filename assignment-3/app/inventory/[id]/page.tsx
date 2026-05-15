import Link from "next/link";
import { ArrowLeft, CalendarClock, Fingerprint, Server } from "lucide-react";
import { notFound } from "next/navigation";
import { Badge } from "@/components/ui/Badge";
import { Card } from "@/components/ui/Card";
import { getCertificate } from "@/lib/certificate-api";
import { certificateState, daysUntil, formatDate } from "@/lib/format";

type PageProps = {
  params: { id: string };
};

export default async function CertificateDetailPage({ params }: Readonly<PageProps>) {
  let certificate;
  try {
    // Missing or unreachable records are mapped to Next's 404 page instead of
    // leaking backend error details into the detail view.
    certificate = await getCertificate(params.id);
  } catch {
    notFound();
  }

  const state = certificateState(certificate.expiration);
  // Badge tone is derived from the same status text used in the table.
  const tone = state === "Expired" ? "danger" : state === "Expiring soon" ? "warning" : "success";

  return (
    <main className="min-h-screen bg-surface">
      <div className="mx-auto flex max-w-5xl flex-col gap-6 px-4 py-6 sm:px-6 lg:px-8">
        <Link className="inline-flex items-center gap-2 text-sm font-medium text-accent" href="/inventory">
          <ArrowLeft className="h-4 w-4" aria-hidden />
          Back to inventory
        </Link>

        <header className="border-b border-line pb-6">
          <div className="mb-3 flex flex-wrap items-center gap-3">
            <Badge tone={tone}>{state}</Badge>
            <span className="text-sm text-slate-500">{daysUntil(certificate.expiration)} days remaining</span>
          </div>
          <h1 className="break-words text-3xl font-semibold tracking-normal text-ink">{certificate.subject}</h1>
          <p className="mt-2 text-sm text-slate-600">{certificate.certificate_id}</p>
        </header>

        <div className="grid gap-4 md:grid-cols-3">
          <Card className="p-4">
            <CalendarClock className="h-5 w-5 text-accent" aria-hidden />
            <p className="mt-3 text-sm font-medium text-slate-500">Expiration</p>
            <p className="mt-1 text-base font-semibold text-ink">{formatDate(certificate.expiration)}</p>
          </Card>
          <Card className="p-4">
            <Server className="h-5 w-5 text-accent" aria-hidden />
            <p className="mt-3 text-sm font-medium text-slate-500">Issuer</p>
            <p className="mt-1 text-base font-semibold text-ink">{certificate.issuer}</p>
          </Card>
          <Card className="p-4">
            <Fingerprint className="h-5 w-5 text-accent" aria-hidden />
            <p className="mt-3 text-sm font-medium text-slate-500">SAN count</p>
            <p className="mt-1 text-base font-semibold text-ink">{certificate.san_entries.length}</p>
          </Card>
        </div>

        <Card className="p-5">
          <h2 className="text-lg font-semibold text-ink">Subject Alternative Names</h2>
          <div className="mt-4 flex flex-wrap gap-2">
            {certificate.san_entries.map((san) => (
              <Badge key={san}>{san}</Badge>
            ))}
          </div>
        </Card>
      </div>
    </main>
  );
}
