"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"
import { formatCurrency } from "@/lib/utils"

export function PortfolioSummary() {
  // Mock data
  const portfolioData = {
    liquid: 8045.32,
    staked: 99798.12,
    locked: {
      total: 27843.56,
      breakdown: [
        { name: "30-day lock", value: 15000.0 },
        { name: "90-day lock", value: 8843.56 },
        { name: "1-year lock", value: 4000.0 },
      ],
    },
  }

  return (
    <Card className="glow-card card-gradient">
      <CardHeader>
        <CardTitle>Portfolio</CardTitle>
        <CardDescription>Your asset breakdown</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex justify-between items-center">
          <span className="text-sm text-muted-foreground">Liquid Assets</span>
          <span className="font-medium">${formatCurrency(portfolioData.liquid)}</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-sm text-muted-foreground">Staked Assets</span>
          <span className="font-medium">${formatCurrency(portfolioData.staked)}</span>
        </div>

        <Separator className="my-2" />

        <div className="space-y-2">
          <div className="flex justify-between items-center">
            <span className="text-sm text-muted-foreground">Time-Locked Assets</span>
            <span className="font-medium">${formatCurrency(portfolioData.locked.total)}</span>
          </div>

          <div className="pl-4 space-y-1">
            {portfolioData.locked.breakdown.map((item, index) => (
              <div key={index} className="flex justify-between items-center text-xs">
                <span className="text-muted-foreground">{item.name}</span>
                <span>${formatCurrency(item.value)}</span>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
