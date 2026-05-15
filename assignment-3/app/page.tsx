import { redirect } from "next/navigation";

export default function Home() {
  // The app is an internal tool, so the root path goes straight to the working
  // inventory surface instead of a marketing page.
  redirect("/inventory");
}
