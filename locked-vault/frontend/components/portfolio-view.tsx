"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Progress } from "@/components/ui/progress"
import { formatCurrency } from "@/lib/utils"

export function PortfolioView() {
  // Mock data
  const portfolioData = {
    totalValue: 135687,
    assets: [
      { name: "Bitcoin", value: 78544.23, change: 2.3, locked: 27843.56, liquid: 50700.67 },
      { name: "Rune Tokens", value: 42456.21, change: -1.7, locked: 15000, liquid: 27456.21 },
      { name: "Ordinals", value: 14686.56, change: 5.8, locked: 8000, liquid: 6686.56 },
    ],
    locks: [
      {
        id: "lock_1",
        asset: "Bitcoin",
        amount: "0.25",
        value: 15000,
        lockPeriod: 90,
        progress: 33,
        remainingDays: 60,
      },
      {
        id: "lock_2",
        asset: "Bitcoin",
        amount: "0.2",
        value: 12843.56,
        lockPeriod: 180,
        progress: 67,
        remainingDays: 60,
      },
      {
        id: "lock_3",
        asset: "Rune",
        amount: "500",
        value: 15000,
        lockPeriod: 30,
        progress: 50,
        remainingDays: 15,
      },
      {
        id: "lock_4",
        asset: "Ordinals",
        amount: "1",
        value: 8000,
        lockPeriod: 365,
        progress: 25,
        remainingDays: 274,
      },
    ],
  }

  return (
    <Tabs defaultValue="overview" className="space-y-6">
      <TabsList className="grid grid-cols-3 w-full max-w-md">
        <TabsTrigger value="overview">Overview</TabsTrigger>
        <TabsTrigger value="assets">Assets</TabsTrigger>
        <TabsTrigger value="locks">Time Locks</TabsTrigger>
      </TabsList>

      <TabsContent value="overview" className="space-y-6">
        <Card className="glow-card card-gradient">
          <CardHeader>
            <CardTitle>Total Value</CardTitle>
            <CardDescription>Your combined asset value</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="text-4xl font-bold">${formatCurrency(portfolioData.totalValue)}</div>
          </CardContent>
        </Card>

        <div className="grid md:grid-cols-2 gap-6">
          <Card className="glow-card card-gradient">
            <CardHeader>
              <CardTitle>Asset Distribution</CardTitle>
              <CardDescription>Breakdown by asset type</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {portfolioData.assets.map((asset, index) => (
                <div key={index} className="space-y-2">
                  <div className="flex justify-between">
                    <span className="font-medium">{asset.name}</span>
                    <span>${formatCurrency(asset.value)}</span>
                  </div>
                  <Progress value={(asset.value / portfolioData.totalValue) * 100} className="h-2" />
                  <div className="flex justify-between text-xs text-muted-foreground">
                    <span>{((asset.value / portfolioData.totalValue) * 100).toFixed(1)}% of portfolio</span>
                    <span className={asset.change >= 0 ? "text-success" : "text-destructive"}>
                      {asset.change >= 0 ? "+" : ""}
                      {asset.change}%
                    </span>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>

          <Card className="glow-card card-gradient">
            <CardHeader>
              <CardTitle>Liquidity Status</CardTitle>
              <CardDescription>Locked vs liquid assets</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {portfolioData.assets.map((asset, index) => (
                <div key={index} className="space-y-2">
                  <div className="flex justify-between">
                    <span className="font-medium">{asset.name}</span>
                    <span>${formatCurrency(asset.value)}</span>
                  </div>
                  <div className="flex h-2 rounded-full overflow-hidden">
                    <div className="bg-primary" style={{ width: `${(asset.locked / asset.value) * 100}%` }}></div>
                    <div className="bg-muted" style={{ width: `${(asset.liquid / asset.value) * 100}%` }}></div>
                  </div>
                  <div className="flex justify-between text-xs text-muted-foreground">
                    <span>Locked: ${formatCurrency(asset.locked)}</span>
                    <span>Liquid: ${formatCurrency(asset.liquid)}</span>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>
        </div>
      </TabsContent>

      <TabsContent value="assets" className="space-y-6">
        <Card className="glow-card card-gradient">
          <CardHeader>
            <CardTitle>Your Assets</CardTitle>
            <CardDescription>Detailed breakdown of your holdings</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-6">
              {portfolioData.assets.map((asset, index) => (
                <div key={index} className="border-b border-border pb-6 last:border-0 last:pb-0">
                  <div className="flex justify-between items-start mb-4">
                    <div>
                      <h3 className="text-xl font-bold">{asset.name}</h3>
                      <p className="text-muted-foreground">${formatCurrency(asset.value)}</p>
                    </div>
                    <div
                      className={`px-2 py-1 rounded text-sm ${
                        asset.change >= 0 ? "bg-success/10 text-success" : "bg-destructive/10 text-destructive"
                      }`}
                    >
                      {asset.change >= 0 ? "+" : ""}
                      {asset.change}%
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div className="space-y-1">
                      <p className="text-sm text-muted-foreground">Locked</p>
                      <p className="font-medium">${formatCurrency(asset.locked)}</p>
                      <p className="text-xs text-muted-foreground">
                        {((asset.locked / asset.value) * 100).toFixed(1)}% of total
                      </p>
                    </div>
                    <div className="space-y-1">
                      <p className="text-sm text-muted-foreground">Liquid</p>
                      <p className="font-medium">${formatCurrency(asset.liquid)}</p>
                      <p className="text-xs text-muted-foreground">
                        {((asset.liquid / asset.value) * 100).toFixed(1)}% of total
                      </p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </TabsContent>

      <TabsContent value="locks" className="space-y-6">
        <Card className="glow-card card-gradient">
          <CardHeader>
            <CardTitle>Time-Locked Deposits</CardTitle>
            <CardDescription>Status of your locked assets</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-6">
              {portfolioData.locks.map((lock) => (
                <div key={lock.id} className="border-b border-border pb-6 last:border-0 last:pb-0">
                  <div className="flex justify-between items-start mb-4">
                    <div>
                      <h3 className="font-bold">
                        {lock.amount} {lock.asset}
                      </h3>
                      <p className="text-sm text-muted-foreground">
                        ${formatCurrency(lock.value)} • {lock.lockPeriod} day lock
                      </p>
                    </div>
                    <div className="text-right">
                      <p className="font-medium">{lock.remainingDays} days left</p>
                      <p className="text-xs text-muted-foreground">{lock.progress}% complete</p>
                    </div>
                  </div>

                  <div className="space-y-2">
                    <Progress value={lock.progress} className="h-2" />
                    <div className="flex justify-between text-xs text-muted-foreground">
                      <span>0 days</span>
                      <span>{lock.lockPeriod} days</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </TabsContent>
    </Tabs>
  )
}
