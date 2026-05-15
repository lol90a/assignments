import { redirect } from "next/navigation";

// The root route redirects to the actual operator workflow. This avoids a
// decorative landing page and makes the application useful immediately.
export default function Home() {
  // The app is an internal tool, so the root path goes straight to the working
  // inventory surface instead of a marketing page.
  redirect("/inventory");
}
