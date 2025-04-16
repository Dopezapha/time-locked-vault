"use client"

import { useState } from "react"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Switch } from "@/components/ui/switch"
import { Label } from "@/components/ui/label"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { Separator } from "@/components/ui/separator"
import { useToast } from "@/components/ui/use-toast"

export function SettingsForm() {
  const [network, setNetwork] = useState("testnet")
  const [notifications, setNotifications] = useState({
    depositConfirmations: true,
    withdrawalConfirmations: true,
    approachingUnlock: true,
    marketUpdates: false,
  })
  const { toast } = useToast()

  const handleSave = () => {
    // Save settings
    toast({
      title: "Settings saved",
      description: "Your settings have been saved successfully.",
    })
  }

  return (
    <Card className="glow-card card-gradient">
      <CardHeader>
        <CardTitle>Application Settings</CardTitle>
        <CardDescription>Manage your preferences</CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-4">
          <h3 className="text-lg font-medium">Network</h3>
          <RadioGroup value={network} onValueChange={setNetwork} className="grid grid-cols-2 gap-4">
            <Label
              htmlFor="testnet"
              className={`flex items-center justify-center gap-2 border rounded-lg p-4 cursor-pointer transition-colors ${
                network === "testnet" ? "bg-primary/10 border-primary" : "border-border hover:border-muted-foreground"
              }`}
            >
              <RadioGroupItem value="testnet" id="testnet" className="sr-only" />
              <span>Testnet</span>
            </Label>
            <Label
              htmlFor="mainnet"
              className={`flex items-center justify-center gap-2 border rounded-lg p-4 cursor-pointer transition-colors ${
                network === "mainnet" ? "bg-primary/10 border-primary" : "border-border hover:border-muted-foreground"
              }`}
            >
              <RadioGroupItem value="mainnet" id="mainnet" className="sr-only" />
              <span>Mainnet</span>
            </Label>
          </RadioGroup>
        </div>

        <Separator />

        <div className="space-y-4">
          <h3 className="text-lg font-medium">Notifications</h3>
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <Label htmlFor="depositConfirmations" className="flex flex-col gap-1">
                <span>Deposit confirmations</span>
                <span className="font-normal text-sm text-muted-foreground">
                  Receive notifications when deposits are confirmed
                </span>
              </Label>
              <Switch
                id="depositConfirmations"
                checked={notifications.depositConfirmations}
                onCheckedChange={(checked) => setNotifications({ ...notifications, depositConfirmations: checked })}
              />
            </div>

            <div className="flex items-center justify-between">
              <Label htmlFor="withdrawalConfirmations" className="flex flex-col gap-1">
                <span>Withdrawal confirmations</span>
                <span className="font-normal text-sm text-muted-foreground">
                  Receive notifications when withdrawals are processed
                </span>
              </Label>
              <Switch
                id="withdrawalConfirmations"
                checked={notifications.withdrawalConfirmations}
                onCheckedChange={(checked) => setNotifications({ ...notifications, withdrawalConfirmations: checked })}
              />
            </div>

            <div className="flex items-center justify-between">
              <Label htmlFor="approachingUnlock" className="flex flex-col gap-1">
                <span>Approaching unlock</span>
                <span className="font-normal text-sm text-muted-foreground">
                  Receive notifications when time-locks are about to expire
                </span>
              </Label>
              <Switch
                id="approachingUnlock"
                checked={notifications.approachingUnlock}
                onCheckedChange={(checked) => setNotifications({ ...notifications, approachingUnlock: checked })}
              />
            </div>

            <div className="flex items-center justify-between">
              <Label htmlFor="marketUpdates" className="flex flex-col gap-1">
                <span>Market updates</span>
                <span className="font-normal text-sm text-muted-foreground">
                  Receive notifications about market changes
                </span>
              </Label>
              <Switch
                id="marketUpdates"
                checked={notifications.marketUpdates}
                onCheckedChange={(checked) => setNotifications({ ...notifications, marketUpdates: checked })}
              />
            </div>
          </div>
        </div>
      </CardContent>
      <CardFooter>
        <Button variant="outline" className="mr-2">
          Reset
        </Button>
        <Button onClick={handleSave}>Save Changes</Button>
      </CardFooter>
    </Card>
  )
}
