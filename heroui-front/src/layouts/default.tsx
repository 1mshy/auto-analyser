import { Navbar } from "@/components/navbar";

export default function DefaultLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="relative flex flex-col min-h-screen bg-gray-50">
      <Navbar />
      <main className="flex-grow">{children}</main>
    </div>
  );
}
