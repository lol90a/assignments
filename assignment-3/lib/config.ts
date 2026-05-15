// Server config is only read from SSR/API routes, keeping backend details out
// of browser bundles.
export const serverConfig = {
  apiBaseUrl: process.env.ASSIGNMENT1_API_BASE_URL ?? "http://127.0.0.1:8080",
  allowSelfSignedCerts: process.env.ALLOW_SELF_SIGNED_CERTS === "true"
};

// Client config is restricted to NEXT_PUBLIC values that are safe to expose.
export const clientConfig = {
  appName: process.env.NEXT_PUBLIC_APP_NAME ?? "Certificate Inventory"
};
