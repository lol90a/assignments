# Frontend Architecture

## Rendering Strategy

The `/inventory` page uses SSR because certificate inventory is operational data that should render predictably, work well behind enterprise gateways, and avoid exposing internal backend topology to the browser. Server components fetch from the Assignment 1 API, then pass safe data into client components.

CSR-only rendering was avoided because it would push backend URLs, token handling, and TLS edge cases into browser JavaScript. That is acceptable for public APIs but weaker for internal platform tooling.

## Data Fetching

`lib/certificate-api.ts` is the only module that knows how to call Assignment 1. This adapter pattern keeps backend contract changes localized. Today it calls Assignment 1's `GET /certificates` endpoint and performs filtering/pagination in the Next.js server layer because the assessment backend is intentionally minimal. The production replacement is a paginated `GET /certificates` contract that filters and pages in PostgreSQL.

SWR is used inside `InventoryTable` for client-side revalidation after the server-rendered response arrives. It is chosen over heavier state management because the UI is read-mostly and the data shape is simple.

## Component Structure

- `DashboardCards` summarizes inventory health.
- `InventoryTable` owns filtering, pagination links, loading, and error states.
- `Card`, `Badge`, and `Skeleton` are reusable UI primitives.
- Detail pages live under `app/inventory/[id]` and use SSR for secure backend access.

## Communication Flow

Browser requests `/inventory`. Next.js runs the page on the server, calls Assignment 1 over HTTPS, renders HTML, and sends the result to the browser. After hydration, SWR calls the local Next.js route `/inventory/api`; that route again calls Assignment 1 from the server side.

This backend-for-frontend pattern keeps the browser talking to one trusted origin and lets the frontend server handle TLS, credentials, and backend topology.
