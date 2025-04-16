"use client"

import type React from "react"

import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { useWallet } from "@/context/wallet-context"
import { Bitcoin, Sparkles, ImageIcon, Zap } from "lucide-react"
import { useToast } from "@/components/ui/use-toast"

export function DepositForm() {
  const { balance, connected } = useWallet()
  const [tokenType, setTokenType] = useState("bitcoin")
  const [amount, setAmount] = useState("")
  const [lockPeriod, setLockPeriod] = useState(30)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const { toast } = useToast()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!connected) {
      toast({
        title: "Wallet not connected",
        description: "Please connect your wallet first",
        variant: "destructive",
      })
      return
    }

    setIsSubmitting(true)

    // Simulate API call
    setTimeout(() => {
      toast({
        title: "Deposit successful",
        description: `Your deposit of ${amount} ${tokenType} for ${lockPeriod} days has been submitted.`,
      })
      setIsSubmitting(false)
      // Reset form
      setAmount("")
    }, 2000)
  }

  const handleMaxClick = () => {
    if (tokenType === "bitcoin") {
      setAmount(balance.toString())
    }
  }

  const getTokenIcon = () => {
    switch (tokenType) {
      case "bitcoin":
        return <Bitcoin className="h-5 w-5" />
      case "rune":
        return <Sparkles className="h-5 w-5" />
      case "ordinals":
        return <ImageIcon className="h-5 w-5" />
      case "lightning":
        return <Zap className="h-5 w-5" />
      default:
        return <Bitcoin className="h-5 w-5" />
    }
  }

  return (
    <Card className="glow-card card-gradient">
      <CardContent className="p-6">
        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-2">
            <Label>Token Type</Label>
            <RadioGroup value={tokenType} onValueChange={setTokenType} className="grid grid-cols-2 gap-4">
              <Label
                htmlFor="bitcoin"
                className={`flex items-center justify-center gap-2 border rounded-lg p-4 cursor-pointer transition-colors ${
                  tokenType === "bitcoin"
                    ? "bg-primary/10 border-primary"
                    : "border-border hover:border-muted-foreground"
                }`}
              >
                <RadioGroupItem value="bitcoin" id="bitcoin" className="sr-only" />
                <Bitcoin className="h-5 w-5" />
                <span>Bitcoin</span>
              </Label>
              <Label
                htmlFor="rune"
                className={`flex items-center justify-center gap-2 border rounded-lg p-4 cursor-pointer transition-colors ${
                  tokenType === "rune" ? "bg-primary/10 border-primary" : "border-border hover:border-muted-foreground"
                }`}
              >
                <RadioGroupItem value="rune" id="rune" className="sr-only" />
                <Sparkles className="h-5 w-5" />
                <span>Rune</span>
              </Label>
              <Label
                htmlFor="ordinals"
                className={`flex items-center justify-center gap-2 border rounded-lg p-4 cursor-pointer transition-colors ${
                  tokenType === "ordinals"
                    ? "bg-primary/10 border-primary"
                    : "border-border hover:border-muted-foreground"
                }`}
              >
                <RadioGroupItem value="ordinals" id="ordinals" className="sr-only" />
                <ImageIcon className="h-5 w-5" />
                <span>Ordinals</span>
              </Label>
              <Label
                htmlFor="lightning"
                className={`flex items-center justify-center gap-2 border rounded-lg p-4 cursor-pointer transition-colors ${
                  tokenType === "lightning"
                    ? "bg-primary/10 border-primary"
                    : "border-border hover:border-muted-foreground"
                }`}
              >
                <RadioGroupItem value="lightning" id="lightning" className="sr-only" />
                <Zap className="h-5 w-5" />
                <span>Lightning</span>
              </Label>
            </RadioGroup>
          </div>

          <div className="space-y-2">
            <div className="flex justify-between">
              <Label htmlFor="amount">Amount</Label>
              {tokenType === "bitcoin" && (
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onClick={handleMaxClick}
                  className="h-auto py-0 px-2 text-xs text-primary"
                >
                  MAX
                </Button>
              )}
            </div>
            <div className="relative">
              <div className="absolute inset-y-0 left-3 flex items-center pointer-events-none">{getTokenIcon()}</div>
              <Input
                id="amount"
                type="number"
                placeholder="0.00"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                className="pl-10"
                step="0.00000001"
                min="0"
                required
              />
            </div>
            {tokenType === "bitcoin" && (
              <p className="text-xs text-muted-foreground">Available: {balance.toFixed(8)} BTC</p>
            )}
          </div>

          <div className="space-y-4">
            <div className="flex justify-between">
              <Label>Lock Period</Label>
              <span className="font-medium">{lockPeriod} days</span>
            </div>
            <Slider
              value={[lockPeriod]}
              onValueChange={(value: number[]) => setLockPeriod(value[0])}
              min={1}
              max={365}
              step={1}
              className="py-4"
            />
            <div className="flex justify-between text-sm text-muted-foreground">
              <span>1 day</span>
              <span>1 year</span>
            </div>
            <div className="flex gap-2 pt-2">
              {[30, 90, 180, 365].map((days) => (
                <Button
                  key={days}
                  type="button"
                  variant={lockPeriod === days ? "secondary" : "outline"}
                  size="sm"
                  onClick={() => setLockPeriod(days)}
                  className="flex-1"
                >
                  {days} days
                </Button>
              ))}
            </div>
          </div>

          <div className="pt-4">
            <Button type="submit" className="w-full" size="lg" disabled={!amount || isSubmitting}>
              {isSubmitting ? (
                <>
                  <div className="h-4 w-4 mr-2 animate-spin rounded-full border-2 border-background border-t-transparent"></div>
                  Processing...
                </>
              ) : (
                "Create Time-Locked Deposit"
              )}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  )
}
