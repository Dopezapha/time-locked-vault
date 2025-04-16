"use client"

import { useState } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Progress } from "@/components/ui/progress"
import { AlertCircle } from "lucide-react"
import { WithdrawConfirmDialog } from "@/components/withdraw-confirm-dialog"

// Mock data
const deposits = [
  {
    id: "dep_1",
    tokenType: "bitcoin",
    amount: "0.25",
    lockPeriod: 90,
    depositDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 30), // 30 days ago
    unlockDate: new Date(Date.now() + 1000 * 60 * 60 * 24 * 60), // 60 days from now
    status: "locked",
  },
  {
    id: "dep_2",
    tokenType: "bitcoin",
    amount: "0.1",
    lockPeriod: 30,
    depositDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 35), // 35 days ago
    unlockDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 5), // 5 days ago
    status: "unlocked",
  },
  {
    id: "dep_3",
    tokenType: "rune",
    amount: "500",
    lockPeriod: 180,
    depositDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 90), // 90 days ago
    unlockDate: new Date(Date.now() + 1000 * 60 * 60 * 24 * 90), // 90 days from now
    status: "locked",
  },
  {
    id: "dep_4",
    tokenType: "ordinals",
    amount: "1",
    lockPeriod: 365,
    depositDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 100), // 100 days ago
    unlockDate: new Date(Date.now() + 1000 * 60 * 60 * 24 * 265), // 265 days from now
    status: "locked",
  },
]

export function WithdrawList() {
  const [selectedDeposit, setSelectedDeposit] = useState<(typeof deposits)[0] | null>(null)
  const [withdrawType, setWithdrawType] = useState<"standard" | "emergency">("standard")
  const [dialogOpen, setDialogOpen] = useState(false)

  const handleWithdraw = (deposit: (typeof deposits)[0], type: "standard" | "emergency") => {
    setSelectedDeposit(deposit)
    setWithdrawType(type)
    setDialogOpen(true)
  }

  const calculateProgress = (deposit: (typeof deposits)[0]) => {
    const totalDuration = deposit.lockPeriod * 24 * 60 * 60 * 1000
    const elapsed = Date.now() - deposit.depositDate.getTime()
    return Math.min(100, Math.round((elapsed / totalDuration) * 100))
  }

  const formatDate = (date: Date) => {
    return date.toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    })
  }

  const getDaysRemaining = (unlockDate: Date) => {
    const diffTime = unlockDate.getTime() - Date.now()
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
    return diffDays
  }

  return (
    <>
      <Card className="glow-card card-gradient">
        <CardHeader>
          <CardTitle>Your Deposits</CardTitle>
          <CardDescription>Manage your time-locked deposits</CardDescription>
        </CardHeader>
        <CardContent>
          {deposits.length === 0 ? (
            <div className="text-center py-8">
              <p className="text-muted-foreground">You don't have any deposits yet.</p>
            </div>
          ) : (
            <div className="space-y-6">
              {deposits.map((deposit) => (
                <div key={deposit.id} className="border border-border rounded-lg p-4 space-y-4">
                  <div className="flex justify-between items-start">
                    <div>
                      <div className="flex items-center gap-2">
                        <h3 className="font-medium">
                          {deposit.amount} {deposit.tokenType.toUpperCase()}
                        </h3>
                        <Badge variant={deposit.status === "unlocked" ? "outline" : "secondary"}>
                          {deposit.status}
                        </Badge>
                      </div>
                      <p className="text-sm text-muted-foreground">
                        {deposit.lockPeriod} day lock • Deposited on {formatDate(deposit.depositDate)}
                      </p>
                    </div>
                    <div className="text-right">
                      {deposit.status === "unlocked" ? (
                        <Button onClick={() => handleWithdraw(deposit, "standard")} size="sm">
                          Withdraw
                        </Button>
                      ) : (
                        <div className="space-y-2">
                          <Button
                            onClick={() => handleWithdraw(deposit, "emergency")}
                            variant="outline"
                            size="sm"
                            className="text-destructive border-destructive hover:bg-destructive/10"
                          >
                            Emergency Withdraw
                          </Button>
                          <p className="text-xs text-muted-foreground">10% penalty fee</p>
                        </div>
                      )}
                    </div>
                  </div>

                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span>Lock progress</span>
                      <span>{calculateProgress(deposit)}%</span>
                    </div>
                    <Progress value={calculateProgress(deposit)} className="h-2" />
                    <div className="flex justify-between text-xs text-muted-foreground">
                      <span>Deposit date</span>
                      <span>Unlock date: {formatDate(deposit.unlockDate)}</span>
                    </div>
                  </div>

                  {deposit.status === "locked" && (
                    <div className="flex items-center gap-2 text-sm text-muted-foreground bg-muted/20 p-2 rounded">
                      <AlertCircle className="h-4 w-4 text-primary" />
                      <span>{getDaysRemaining(deposit.unlockDate)} days remaining until unlock</span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      <WithdrawConfirmDialog
        deposit={selectedDeposit}
        withdrawType={withdrawType}
        open={dialogOpen}
        onOpenChange={setDialogOpen}
      />
    </>
  )
}
