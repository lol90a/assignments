import { NextRequest, NextResponse } from "next/server";
import { listCertificates } from "@/lib/certificate-api";

// This API route is a small backend-for-frontend layer. It gives SWR a
// same-origin URL after hydration while keeping the Assignment 1 API base URL
// and TLS handling inside server-only code.
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
