import { Sidebar } from "@/components/sidebar"
import { Navbar } from "@/components/navbar"
import { OrbitalBackground } from "@/components/orbital-background"
import { WithdrawList } from "@/components/withdraw-list"

export default function WithdrawPage() {
  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar className="hidden md:flex" />
      <div className="flex flex-col flex-1 overflow-hidden">
        <Navbar />
        <main className="flex-1 overflow-y-auto p-6 relative">
          <OrbitalBackground />
          <div className="container max-w-4xl mx-auto">
            <h1 className="text-3xl font-bold mb-6">Withdraw Funds</h1>
            <WithdrawList />
          </div>
        </main>
      </div>
    </div>
  )
}