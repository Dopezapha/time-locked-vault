"use client"

import { Card, CardContent } from "@/components/ui/card"
import { useWallet } from "@/context/wallet-context"
import { formatCurrency } from "@/lib/utils"
import { Bitcoin } from "lucide-react"

export function BalanceDisplay() {
  const { balance } = useWallet()

  // Convert BTC to USD (mock value)
  const btcPrice = 65432.1
  const usdValue = balance * btcPrice

  return (
    <Card className="glow-card card-gradient">
      <CardContent className="p-6">
        <div className="flex items-center justify-between">
          <div className="space-y-1">
            <p className="text-sm font-medium text-muted-foreground">Total Balance</p>
            <h2 className="text-4xl font-bold tracking-tight">${formatCurrency(usdValue)}</h2>
          </div>
          <div className="text-right flex items-center gap-2">
            <Bitcoin className="h-5 w-5 text-primary" />
            <div>
              <p className="text-sm font-medium text-muted-foreground">BTC</p>
              <p className="text-xl font-semibold">{balance.toFixed(8)}</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
