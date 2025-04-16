import Link from "next/link"
import { WalletButton } from "@/components/wallet-button"
import { NetworkIndicator } from "@/components/network-indicator"
import { UserNav } from "@/components/user-nav"

export function Navbar() {
  return (
    <header className="border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-16 items-center justify-between">
        <div className="flex items-center gap-6">
          <Link href="/" className="flex items-center gap-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="h-6 w-6 text-primary"
            >
              <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
            </svg>
            <span className="text-xl font-bold">BitLock</span>
          </Link>
          <nav className="hidden md:flex gap-6">
            <Link href="/dashboard" className="text-sm font-medium hover:text-primary transition-colors">
              Dashboard
            </Link>
            <Link href="/dashboard/deposit" className="text-sm font-medium hover:text-primary transition-colors">
              Deposit
            </Link>
            <Link href="/dashboard/withdraw" className="text-sm font-medium hover:text-primary transition-colors">
              Withdraw
            </Link>
            <Link href="/dashboard/portfolio" className="text-sm font-medium hover:text-primary transition-colors">
              Portfolio
            </Link>
            <Link href="/dashboard/settings" className="text-sm font-medium hover:text-primary transition-colors">
              Settings
            </Link>
            <Link href="/docs" className="text-sm font-medium hover:text-primary transition-colors">
              Docs
            </Link>
          </nav>
        </div>
        <div className="flex items-center gap-4">
          <NetworkIndicator />
          <WalletButton />
          <UserNav />
        </div>
      </div>
    </header>
  )
}
