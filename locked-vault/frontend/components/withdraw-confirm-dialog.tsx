"use client"

import { useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { AlertTriangle } from "lucide-react"
import { useToast } from "@/components/ui/use-toast"

interface WithdrawConfirmDialogProps {
  deposit: any
  withdrawType: "standard" | "emergency"
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function WithdrawConfirmDialog({ deposit, withdrawType, open, onOpenChange }: WithdrawConfirmDialogProps) {
  const [isSubmitting, setIsSubmitting] = useState(false)
  const { toast } = useToast()

  if (!deposit) return null

  const handleConfirm = async () => {
    setIsSubmitting(true)

    // Simulate API call
    setTimeout(() => {
      setIsSubmitting(false)
      onOpenChange(false)
      toast({
        title: "Withdrawal successful",
        description: `Your withdrawal of ${deposit.amount} ${deposit.tokenType.toUpperCase()} has been processed.`,
      })
    }, 2000)
  }

  const calculatePenalty = () => {
    if (withdrawType === "emergency") {
      return Number.parseFloat(deposit.amount) * 0.1
    }
    return 0
  }

  const finalAmount = Number.parseFloat(deposit.amount) - calculatePenalty()

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{withdrawType === "standard" ? "Withdraw Funds" : "Emergency Withdrawal"}</DialogTitle>
          <DialogDescription>
            {withdrawType === "standard"
              ? "Confirm your withdrawal from the time-locked deposit."
              : "Early withdrawal will incur a 10% penalty fee."}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="space-y-2">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Deposit amount:</span>
              <span>
                {deposit.amount} {deposit.tokenType.toUpperCase()}
              </span>
            </div>

            {withdrawType === "emergency" && (
              <div className="flex justify-between text-destructive">
                <span>Penalty fee (10%):</span>
                <span>
                  -{calculatePenalty()} {deposit.tokenType.toUpperCase()}
                </span>
              </div>
            )}

            <div className="flex justify-between font-medium pt-2 border-t">
              <span>You will receive:</span>
              <span>
                {finalAmount} {deposit.tokenType.toUpperCase()}
              </span>
            </div>
          </div>

          {withdrawType === "emergency" && (
            <div className="flex items-center gap-2 p-3 bg-destructive/10 text-destructive rounded-md">
              <AlertTriangle className="h-5 w-5" />
              <div className="text-sm">
                <p className="font-medium">Warning: Early Withdrawal</p>
                <p>You are withdrawing before the lock period ends. This action cannot be undone.</p>
              </div>
            </div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleConfirm}
            disabled={isSubmitting}
            variant={withdrawType === "emergency" ? "destructive" : "default"}
          >
            {isSubmitting ? (
              <>
                <div className="h-4 w-4 mr-2 animate-spin rounded-full border-2 border-background border-t-transparent"></div>
                Processing...
              </>
            ) : (
              "Confirm Withdrawal"
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
