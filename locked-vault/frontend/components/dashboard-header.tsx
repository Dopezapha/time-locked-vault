"use client"

import Link from "next/link"
import { WalletButton } from "@/components/wallet-button"
import { UserNav } from "@/components/user-nav"
import { Button } from "@/components/ui/button"
import { Menu, ChevronLeft, ChevronRight } from "lucide-react"
import { usePathname } from "next/navigation"
import { cn } from "@/lib/utils"

interface DashboardHeaderProps {
  sidebarOpen: boolean
  onSidebarToggle: () => void
  onMobileSidebarToggle: () => void
}

export function DashboardHeader({ sidebarOpen, onSidebarToggle, onMobileSidebarToggle }: DashboardHeaderProps) {
  const pathname = usePathname()

  const navItems = [
    { name: "Dashboard", path: "/dashboard" },
    { name: "Deposit", path: "/dashboard/deposit" },
    { name: "Withdraw", path: "/dashboard/withdraw" },
    { name: "Portfolio", path: "/dashboard/portfolio" },
    { name: "Settings", path: "/dashboard/settings" },
    { name: "Docs", path: "/docs" },
  ]

  return (
    <>
      <header className="dashboard-header border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 z-10">
        <div className="flex h-16 items-center justify-between px-4">
          <div className="flex items-center gap-4">
            <Button variant="ghost" size="icon" onClick={onSidebarToggle} className="hidden md:flex">
              {sidebarOpen ? <ChevronLeft size={18} /> : <ChevronRight size={18} />}
            </Button>

            <Button variant="ghost" size="icon" onClick={onMobileSidebarToggle} className="md:hidden">
              <Menu size={20} />
            </Button>

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
          </div>

          {/* Desktop navigation links */}
          <nav className="hidden md:flex items-center gap-6">
            {navItems.map((item) => (
              <Link
                key={item.path}
                href={item.path}
                className={cn(
                  "text-sm font-medium transition-colors",
                  pathname === item.path ? "text-primary" : "text-muted-foreground hover:text-primary",
                )}
              >
                {item.name}
              </Link>
            ))}
          </nav>

          <div className="flex items-center gap-4">
            <WalletButton />
            <UserNav />
          </div>
        </div>
      </header>

      {/* Mobile horizontal navigation */}
      <div className="md:hidden w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 shadow-sm">
        <div className="flex overflow-x-auto py-3 px-4 gap-6 no-scrollbar">
          {navItems.map((item) => (
            <Link
              key={item.path}
              href={item.path}
              className={cn(
                "text-sm whitespace-nowrap font-medium transition-colors flex-shrink-0",
                pathname === item.path
                  ? "text-primary border-b-2 border-primary pb-1"
                  : "text-muted-foreground hover:text-primary",
              )}
            >
              {item.name}
            </Link>
          ))}
        </div>
      </div>
    </>
  )
}
