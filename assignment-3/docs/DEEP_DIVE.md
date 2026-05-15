# Assignment 3 Deep Dive

This document explains Assignment 3 in detail so you can walk through it confidently during a technical interview.

## Goal

Assignment 3 is a Next.js operational frontend for certificate inventory. It integrates with Assignment 1 backend and demonstrates SSR, secure server-side API consumption, SWR revalidation, and reusable UI composition.

## App Structure

### `app/layout.tsx`

Role:

- Root HTML/body shell and metadata.

Why:

- Provides consistent app framing and metadata for all routes.

### `app/page.tsx`

Role:

- Redirects root path to `/inventory`.

Why:

- Keeps inventory as primary landing route for operator workflow.

### `app/inventory/page.tsx`

Role:

- SSR inventory page.
- Loads paginated inventory data server-side.
- Loads summary cards data.

Why SSR:

- Keeps backend URL and transport settings server-side.
- Better first render for enterprise dashboards.

### `app/inventory/[id]/page.tsx`

Role:

- SSR detail page for one certificate.

Why:

- Secure server-side fetch.
- Better direct-link usability for operators.

### `app/inventory/api/route.ts`

Role:

- Backend-for-frontend route used by SWR on client side.

Why:

- Browser talks only to Next.js origin.
- Backend topology and credentials remain server-side.

### `app/inventory/loading.tsx`

Role:

- Route-level loading UI while server/client data settles.

### `app/inventory/error.tsx`

Role:

- Route-level error boundary with retry path.

## UI Components

### `components/inventory/DashboardCards.tsx`

- Displays high-level metrics: total, expiring soon, issuers, expired.

### `components/inventory/InventoryTable.tsx`

- Table rendering, filter input, navigation links, SWR data refresh.

### `components/ui/Card.tsx`, `Badge.tsx`, `Skeleton.tsx`

- Reusable primitives for consistent design and lower duplication.

## Data Layer

### `lib/config.ts`

- Server and client configuration parsing.
- Reads `ASSIGNMENT1_API_BASE_URL` and TLS toggles.

### `lib/types.ts`

- Shared TypeScript contracts for certificates and inventory payloads.

### `lib/certificate-api.ts`

Main responsibilities:

- Server-side calls to Assignment 1 API.
- Local self-signed TLS compatibility mode.
- Calls the Assignment 1 list endpoint, then applies filtering and pagination in the Next.js server layer.
- Dashboard summary aggregation.

Critical design note:

- Assignment 1 exposes `GET /certificates`, which is enough for this assessment.
- Filtering and pagination are still performed in the frontend server layer because the Assignment 1 API is intentionally minimal.
- In production, filtering and pagination should move into PostgreSQL-backed backend query parameters.

### `lib/format.ts`

- Presentation formatting helpers (date, days-left, health state).

Why:

- Prevents formatting duplication across components.

## Security Design

- Browser does not directly call Assignment 1.
- Next.js server performs backend calls.
- TLS behavior controlled by environment and defaults to secure mode.
- Self-signed bypass is explicit local-development only toggle.

## SSR vs CSR Tradeoff

Why SSR chosen:

- Better first paint and operational reliability.
- Sensitive backend settings remain server-only.

Why SWR still used:

- Enables in-place refresh and responsive UI after hydration.
- Keeps client state management lightweight.

## Operational Notes

Common confusion point:

- If `.env.local` is missing or `ASSIGNMENT1_API_BASE_URL` points at the wrong host, the table cannot load data.
- Env changes require Next.js server restart.
- Query parameters like `expiresWithinDays=30` can filter out long-lived certs while summary still shows total.

## Interview Narrative

"I used Next.js App Router with SSR for secure operational views, then layered SWR for client-side freshness. I isolated backend integration in a dedicated API module and a BFF route so browser exposure is minimized. Assignment 1 now provides a minimal list endpoint, while the frontend documents why true production pagination and filtering should move into backend SQL queries."
