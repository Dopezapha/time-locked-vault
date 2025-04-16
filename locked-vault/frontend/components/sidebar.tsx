"use client"

import { useState } from "react"
import Link from "next/link"
import { usePathname } from "next/navigation"
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import {
  LayoutDashboard,
  ArrowDownToLine,
  ArrowUpFromLine,
  PieChart,
  Settings,
  ChevronRight,
  ChevronLeft,
} from "lucide-react"

interface SidebarProps {
  className?: string
}

export function Sidebar({ className }: SidebarProps) {
  const pathname = usePathname()
  const [expanded, setExpanded] = useState(true)

  const toggleSidebar = () => {
    setExpanded(!expanded)
  }

  return (
    <div
      className={cn(
        "flex flex-col h-screen border-r border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 transition-all duration-300",
        expanded ? "w-64" : "w-20",
        className,
      )}
    >
      <div className="flex items-center justify-between p-4 h-16 border-b border-border/40">
        {expanded ? (
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
        ) : (
          <Link href="/" className="mx-auto">
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
          </Link>
        )}
        <Button
          variant="ghost"
          size="icon"
          onClick={toggleSidebar}
          className="text-muted-foreground hover:text-foreground"
        >
          {expanded ? <ChevronLeft size={18} /> : <ChevronRight size={18} />}
        </Button>
      </div>
      <nav className="flex-1 p-4 space-y-2">
        <Link href="/" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/" ? "bg-muted text-foreground" : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <LayoutDashboard size={20} />
            {expanded && <span>Dashboard</span>}
          </Button>
        </Link>
        <Link href="/deposit" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/deposit" ? "bg-muted text-foreground" : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <ArrowDownToLine size={20} />
            {expanded && <span>Deposit</span>}
          </Button>
        </Link>
        <Link href="/withdraw" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/withdraw" ? "bg-muted text-foreground" : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <ArrowUpFromLine size={20} />
            {expanded && <span>Withdraw</span>}
          </Button>
        </Link>
        <Link href="/portfolio" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/portfolio" ? "bg-muted text-foreground" : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <PieChart size={20} />
            {expanded && <span>Portfolio</span>}
          </Button>
        </Link>
        <Link href="/settings" passHref>
          <Button
            variant="ghost"
            className={cn(
              "w-full justify-start gap-3 font-normal",
              pathname === "/settings" ? "bg-muted text-foreground" : "text-muted-foreground hover:text-foreground",
              !expanded && "justify-center px-2",
            )}
          >
            <Settings size={20} />
            {expanded && <span>Settings</span>}
          </Button>
        </Link>
      </nav>
    </div>
  )
}
