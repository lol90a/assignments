# Assignment 3: Next.js Certificate Inventory Frontend

This is a professional Next.js App Router frontend that integrates with the Assignment 1 Rust certificate backend. It provides an `/inventory` page with dashboard cards, certificate listing, details, filtering, pagination, loading states, error states, reusable components, TailwindCSS styling, strict TypeScript, SSR, and SWR revalidation.

## Important Backend Contract Note

Assignment 1 exposes:

- `GET /health`
- `POST /certificates`
- `GET /certificates`
- `GET /certificates/:id`
- `POST /parse-certificate`

The frontend uses `GET /certificates` for the inventory table and `GET /certificates/:id` for detail pages. Filtering and pagination currently happen in the Next.js server layer because Assignment 1 is intentionally minimal; a production backend should push those operations into PostgreSQL with indexed query parameters.

## Getting Started

```bash
npm install
cp .env.example .env.local
npm run dev
```

Create certificates through Assignment 1, then open:

```text
http://localhost:3000/inventory
```

## Environment Variables

- `ASSIGNMENT1_API_BASE_URL`: Backend base URL, for example `https://127.0.0.1:8080`.
- `ALLOW_SELF_SIGNED_CERTS`: Local-only switch for self-signed TLS development.
- `NEXT_PUBLIC_APP_NAME`: Browser-visible display name.

## Architecture

- `app/inventory/page.tsx` is an SSR route. It fetches inventory on the server so backend URLs and credentials stay off the browser.
- `app/inventory/api/route.ts` is a backend-for-frontend route used by SWR after hydration.
- `components/inventory` contains reusable inventory UI.
- `components/ui` contains low-level presentation primitives.
- `lib/certificate-api.ts` isolates Assignment 1 API calls and TLS behavior.
- `lib/format.ts` centralizes date/state formatting.

## Why SSR and SWR

SSR gives a fast first meaningful render, keeps sensitive backend configuration server-side, and makes the page resilient in restricted enterprise browsers. SWR is used after hydration for lightweight revalidation and client-side refresh behavior. React Query would also work, but SWR is smaller and well-suited to read-mostly dashboard pages with simple cache keys.

## TLS and Local Development

See `docs/TLS_AND_MTLS.md` for self-signed TLS and local mTLS explanation. In production, use trusted certificates at the edge and service mesh mTLS inside Kubernetes.

## Production Considerations

- Add authentication through OIDC and pass only server-side tokens to backend calls.
- Move filtering and pagination into Assignment 1 once the backend grows beyond the minimal assessment API.
- Add CSP headers once final API/image/font origins are known.
- Run with a non-root container user.
- Use mTLS inside Kubernetes through Istio or Linkerd rather than hand-rolling peer certificate validation in the frontend.
