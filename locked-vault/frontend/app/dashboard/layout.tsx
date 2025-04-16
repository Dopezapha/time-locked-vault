"use client"

import type React from "react"

import { useState } from "react"
import { DashboardSidebar } from "@/components/dashboard-sidebar"
import { DashboardHeader } from "@/components/dashboard-header"
import { OrbitalBackground } from "@/components/orbital-background"
import { cn } from "@/lib/utils"

export default function DashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const [mobileSidebarOpen, setMobileSidebarOpen] = useState(false)

  return (
    <div className="dashboard-layout">
      <DashboardHeader
        sidebarOpen={sidebarOpen}
        onSidebarToggle={() => setSidebarOpen(!sidebarOpen)}
        onMobileSidebarToggle={() => setMobileSidebarOpen(!mobileSidebarOpen)}
      />

      <DashboardSidebar
        className={cn("dashboard-sidebar", mobileSidebarOpen && "mobile-open")}
        expanded={sidebarOpen}
        onClose={() => setMobileSidebarOpen(false)}
      />

      <main className="dashboard-main overflow-y-auto relative">
        <OrbitalBackground density="low" />
        <div className="p-6">{children}</div>
      </main>
    </div>
  )
}