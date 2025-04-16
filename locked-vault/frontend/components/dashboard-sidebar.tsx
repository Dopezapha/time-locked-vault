"use client"

import { useEffect } from "react"
import Link from "next/link"
import { usePathname } from "next/navigation"
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { LayoutDashboard, ArrowDownToLine, ArrowUpFromLine, PieChart, Settings, X, FileText } from "lucide-react"

interface DashboardSidebarProps {
  className?: string
  expanded: boolean
  onClose: () => void
}

export function DashboardSidebar({ className, expanded, onClose }: DashboardSidebarProps) {
  const pathname = usePathname()

  // Close sidebar when clicking outside on mobile
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      const sidebar = document.querySelector(".dashboard-sidebar.mobile-open")
      if (sidebar && !sidebar.contains(event.target as Node)) {
        onClose()
      }
    }

    document.addEventListener("mousedown", handleClickOutside)
    return () => {
      document.removeEventListener("mousedown", handleClickOutside)
    }
  }, [onClose])

  return (
    <div
      className={cn(
        "flex flex-col h-full border-r border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 transition-all duration-300",
        expanded ? "w-56" : "w-16",
        className,
      )}
    >
      <div className="flex items-center justify-between p-4 h-16 border-b border-border/40 md:hidden">
        <div className="flex-1"></div>
        <Button variant="ghost" size="icon" onClick={onClose} className="focus:outline-none">
          <X size={18} />
        </Button>
      </div>

      <div className="items-center justify-center p-4 h-16 border-b border-border/40 hidden md:flex">
      </div>

      <nav className="flex-1 p-4 space-y-2 overflow-y-auto">
        <Link href="/dashboard" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/dashboard" ? "bg-muted text-foreground" : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <LayoutDashboard size={20} />
            {expanded && <span>Dashboard</span>}
          </Button>
        </Link>
        <Link href="/dashboard/deposit" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/dashboard/deposit"
                ? "bg-muted text-foreground"
                : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <ArrowDownToLine size={20} />
            {expanded && <span>Deposit</span>}
          </Button>
        </Link>
        <Link href="/dashboard/withdraw" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/dashboard/withdraw"
                ? "bg-muted text-foreground"
                : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <ArrowUpFromLine size={20} />
            {expanded && <span>Withdraw</span>}
          </Button>
        </Link>
        <Link href="/dashboard/portfolio" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/dashboard/portfolio"
                ? "bg-muted text-foreground"
                : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <PieChart size={20} />
            {expanded && <span>Portfolio</span>}
          </Button>
        </Link>
        <Link href="/dashboard/settings" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/dashboard/settings"
                ? "bg-muted text-foreground"
                : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <Settings size={20} />
            {expanded && <span>Settings</span>}
          </Button>
        </Link>
        <Link href="/docs" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/docs" ? "bg-muted text-foreground" : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <FileText size={20} />
            {expanded && <span>Docs</span>}
          </Button>
        </Link>
      </nav>

      <div className="p-4 border-t border-border/40">
        <div className={cn("rounded-lg bg-muted/30 p-4", !expanded && "text-center")}>
          {expanded ? (
            <>
              <p className="text-sm font-medium mb-2">Need Help?</p>
              <p className="text-xs text-muted-foreground mb-3">Check our documentation</p>
              <Link href="/docs">
                <Button size="sm" variant="outline" className="w-full">
                  View Docs
                </Button>
              </Link>
            </>
          ) : (
            <Link href="/docs">
              <Button size="icon" variant="outline">
                ?
              </Button>
            </Link>
          )}
        </div>
      </div>
    </div>
  )
}
