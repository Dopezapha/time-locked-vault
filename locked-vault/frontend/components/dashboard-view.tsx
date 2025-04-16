"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { ArrowDownToLine, ArrowUpFromLine, TrendingUp, Clock, ExternalLink } from "lucide-react"
import { useWallet } from "@/context/wallet-context"
import { BalanceDisplay } from "@/components/balance-display"
import { PortfolioSummary } from "@/components/portfolio-summary"
import { RecentActivity } from "@/components/recent-activity"
import Link from "next/link"
import { WalletButton } from "@/components/wallet-button"

export function DashboardView() {
  const { connected } = useWallet()

  if (!connected) {
    return (
      <div className="container max-w-6xl mx-auto mt-10 text-center">
        <h1 className="text-4xl font-bold tracking-tight mb-6">
          Bitcoin Time-Locked <span className="text-gradient">Deposits</span>
        </h1>
        <p className="text-xl text-muted-foreground mb-10 max-w-2xl mx-auto">
          Secure your Bitcoin, Rune tokens, Ordinals, and Lightning Network payments with configurable time-lock
          periods.
        </p>
        <div className="flex flex-col items-center gap-4">
          <WalletButton />
          <p className="text-sm text-muted-foreground">Connect your wallet to get started</p>
        </div>
      </div>
    )
  }

  return (
    <div className="container max-w-6xl mx-auto space-y-8">
      <BalanceDisplay />

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 card-grid">
        <PortfolioSummary />

        <Card className="glow-card card-gradient">
          <CardHeader>
            <CardTitle>Quick Actions</CardTitle>
            <CardDescription>Manage your time-locked deposits</CardDescription>
          </CardHeader>
          <CardContent className="grid grid-cols-2 gap-4">
            <Link href="/dashboard/deposit" passHref>
              <Button className="w-full h-20 sm:h-24 flex flex-col items-center justify-center gap-2">
                <ArrowDownToLine className="h-5 w-5 sm:h-6 sm:w-6" />
                <span>Deposit</span>
              </Button>
            </Link>
            <Link href="/dashboard/withdraw" passHref>
              <Button variant="outline" className="w-full h-20 sm:h-24 flex flex-col items-center justify-center gap-2">
                <ArrowUpFromLine className="h-5 w-5 sm:h-6 sm:w-6" />
                <span>Withdraw</span>
              </Button>
            </Link>
            <Link href="/dashboard/portfolio" passHref>
              <Button variant="outline" className="w-full h-20 sm:h-24 flex flex-col items-center justify-center gap-2">
                <TrendingUp className="h-5 w-5 sm:h-6 sm:w-6" />
                <span>Portfolio</span>
              </Button>
            </Link>
            <Link href="/dashboard/locks" passHref>
              <Button variant="outline" className="w-full h-20 sm:h-24 flex flex-col items-center justify-center gap-2">
                <Clock className="h-5 w-5 sm:h-6 sm:w-6" />
                <span>Time Locks</span>
              </Button>
            </Link>
          </CardContent>
        </Card>
      </div>

      <RecentActivity />

      <Card className="glow-card card-gradient overflow-hidden">
        <CardHeader className="pb-0">
          <CardTitle>Market Overview</CardTitle>
          <CardDescription>Latest Bitcoin market data</CardDescription>
        </CardHeader>
        <CardContent className="p-0">
          <div className="p-6 pt-4">
            <div className="flex justify-between items-center mb-4">
              <div>
                <h3 className="text-2xl font-bold">$65,432.10</h3>
                <p className="text-sm text-muted-foreground">Bitcoin Price</p>
              </div>
              <div className="text-right">
                <span className="px-2 py-1 text-sm bg-success/10 text-success rounded">+2.4%</span>
                <p className="text-xs text-muted-foreground mt-1">24h change</p>
              </div>
            </div>

            <div className="h-40 w-full bg-muted/20 rounded-lg flex items-center justify-center">
              <p className="text-muted-foreground">Price chart will appear here</p>
            </div>

            <div className="flex justify-between mt-4 text-sm">
              <div>
                <p className="text-muted-foreground">24h Volume</p>
                <p className="font-medium">$24.5B</p>
              </div>
              <div>
                <p className="text-muted-foreground">Market Cap</p>
                <p className="font-medium">$1.2T</p>
              </div>
              <div>
                <p className="text-muted-foreground">Dominance</p>
                <p className="font-medium">52.3%</p>
              </div>
            </div>
          </div>

          <div className="flex items-center justify-center border-t border-border/40 p-3">
            <Link
              href="https://www.coingecko.com/en/coins/bitcoin"
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-primary flex items-center"
            >
              View full market data <ExternalLink className="ml-1 h-3 w-3" />
            </Link>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
