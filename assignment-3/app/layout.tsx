import type { Metadata } from "next";
import "./globals.css";

// App-level metadata is intentionally plain and operational. This is an
// internal inventory tool, so marketing language would add noise instead of
// helping operators understand the surface.
export const metadata: Metadata = {
  title: "Certificate Inventory",
  description: "Enterprise certificate inventory frontend for the Rust certificate backend"
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  // Keep the root layout minimal so route pages control their own operational
  // surfaces and loading/error states.
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
