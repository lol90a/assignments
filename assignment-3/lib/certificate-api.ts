import https from "node:https";
import { differenceInCalendarDays, isBefore, parseISO } from "date-fns";
import { serverConfig } from "./config";
import type { Certificate, DashboardSummary, InventoryFilters, InventoryResult } from "./types";

const localSelfSignedAgent =
  serverConfig.allowSelfSignedCerts && serverConfig.apiBaseUrl.startsWith("https://")
    ? new https.Agent({ rejectUnauthorized: false })
    : undefined;

async function backendFetch<T>(path: string): Promise<T> {
  // Local demos often use a self-signed certificate. When explicitly enabled,
  // fall back to a Node HTTPS adapter that can use a relaxed TLS agent.
  if (localSelfSignedAgent) {
    return backendFetchWithHttpsAgent<T>(path, localSelfSignedAgent);
  }

  const response = await fetch(`${serverConfig.apiBaseUrl}${path}`, {
    // SSR uses Node fetch on the server. Keeping this adapter server-only avoids
    // exposing internal backend URLs or service tokens to the browser.
    cache: "no-store"
  });

  if (!response.ok) {
    throw new Error(`Assignment 1 backend returned ${response.status} for ${path}`);
  }

  return (await response.json()) as T;
}

function backendFetchWithHttpsAgent<T>(path: string, agent: https.Agent): Promise<T> {
  // Native fetch does not accept Node's custom HTTPS agent in this runtime, so
  // this helper keeps the TLS exception isolated to server-side demo calls.
  return new Promise((resolve, reject) => {
    const request = https.get(`${serverConfig.apiBaseUrl}${path}`, { agent }, (response) => {
      let body = "";
      response.setEncoding("utf8");
      response.on("data", (chunk) => {
        body += chunk;
      });
      response.on("end", () => {
        if (!response.statusCode || response.statusCode < 200 || response.statusCode >= 300) {
          reject(new Error(`Assignment 1 backend returned ${response.statusCode} for ${path}`));
          return;
        }

        try {
          resolve(JSON.parse(body) as T);
        } catch (error) {
          reject(error);
        }
      });
    });

    request.on("error", reject);
    request.setTimeout(10_000, () => {
      request.destroy(new Error(`Timed out calling Assignment 1 backend for ${path}`));
    });
  });
}

export async function getCertificate(id: string): Promise<Certificate> {
  return backendFetch<Certificate>(`/certificates/${encodeURIComponent(id)}`);
}

export async function listCertificates(filters: InventoryFilters): Promise<InventoryResult> {
  // Assignment 1 exposes a real list endpoint. The frontend still performs
  // filtering and pagination here because the assessment API keeps the backend
  // intentionally minimal.
  const certificates = await backendFetch<Certificate[]>("/certificates");
  const query = filters.query?.toLowerCase().trim();
  const now = new Date();

  const filtered = certificates
    .filter((certificate) => {
      if (!query) return true;
      return (
        certificate.subject.toLowerCase().includes(query) ||
        certificate.issuer.toLowerCase().includes(query) ||
        certificate.san_entries.some((san) => san.toLowerCase().includes(query))
      );
    })
    .filter((certificate) => {
      if (!filters.expiresWithinDays) return true;
      const days = differenceInCalendarDays(parseISO(certificate.expiration), now);
      return days >= 0 && days <= filters.expiresWithinDays;
    })
    .sort((a, b) => parseISO(a.expiration).getTime() - parseISO(b.expiration).getTime());

  const start = (filters.page - 1) * filters.pageSize;
  return {
    items: filtered.slice(start, start + filters.pageSize),
    total: filtered.length,
    page: filters.page,
    pageSize: filters.pageSize
  };
}

export function summarizeInventory(certificates: Certificate[]): DashboardSummary {
  // Summary metrics are derived from the visible certificate set so the cards
  // and table stay consistent during the assignment integration.
  const now = new Date();
  const issuers = new Set(certificates.map((certificate) => certificate.issuer));

  return {
    total: certificates.length,
    expiringSoon: certificates.filter((certificate) => {
      const days = differenceInCalendarDays(parseISO(certificate.expiration), now);
      return days >= 0 && days <= 30;
    }).length,
    uniqueIssuers: issuers.size,
    expired: certificates.filter((certificate) => isBefore(parseISO(certificate.expiration), now)).length
  };
}
