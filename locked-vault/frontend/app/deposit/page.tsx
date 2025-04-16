import { Sidebar } from "@/components/sidebar"
import { Navbar } from "@/components/navbar"
import { OrbitalBackground } from "@/components/orbital-background"
import { DepositForm } from "@/components/deposit-form"

export default function DepositPage() {
  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar className="hidden md:flex" />
      <div className="flex flex-col flex-1 overflow-hidden">
        <Navbar />
        <main className="flex-1 overflow-y-auto p-6 relative">
          <OrbitalBackground />
          <div className="container max-w-2xl mx-auto">
            <h1 className="text-3xl font-bold mb-6">Create Time-Locked Deposit</h1>
            <DepositForm />
          </div>
        </main>
      </div>
    </div>
  )
}