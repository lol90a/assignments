import { NextRequest, NextResponse } from "next/server";
import { listCertificates } from "@/lib/certificate-api";

export async function GET(request: NextRequest) {
  // This route gives the client table a same-origin endpoint for SWR refreshes
  // while the backend service URL remains server-only.
  const { searchParams } = request.nextUrl;
  const result = await listCertificates({
    query: searchParams.get("query") ?? undefined,
    expiresWithinDays: searchParams.get("expiresWithinDays")
      ? Number(searchParams.get("expiresWithinDays"))
      : undefined,
    page: Number(searchParams.get("page") ?? "1"),
    pageSize: Number(searchParams.get("pageSize") ?? "10")
  });

  return NextResponse.json(result);
}
