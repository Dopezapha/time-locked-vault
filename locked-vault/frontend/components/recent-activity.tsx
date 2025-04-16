import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { formatDistanceToNow } from "date-fns"

export function RecentActivity() {
  // Mock data
  const activities = [
    {
      id: 1,
      type: "deposit",
      amount: "0.25 BTC",
      lockPeriod: "90 days",
      timestamp: new Date(Date.now() - 1000 * 60 * 30), // 30 minutes ago
      status: "confirmed",
    },
    {
      id: 2,
      type: "withdraw",
      amount: "0.1 BTC",
      lockPeriod: "30 days",
      timestamp: new Date(Date.now() - 1000 * 60 * 60 * 2), // 2 hours ago
      status: "confirmed",
    },
    {
      id: 3,
      type: "emergency_withdraw",
      amount: "0.05 BTC",
      lockPeriod: "1 year",
      timestamp: new Date(Date.now() - 1000 * 60 * 60 * 24), // 1 day ago
      status: "confirmed",
    },
    {
      id: 4,
      type: "deposit",
      amount: "0.15 BTC",
      lockPeriod: "30 days",
      timestamp: new Date(Date.now() - 1000 * 60 * 60 * 24 * 3), // 3 days ago
      status: "confirmed",
    },
  ]

  return (
    <Card className="glow-card card-gradient">
      <CardHeader>
        <CardTitle>Recent Activity</CardTitle>
        <CardDescription>Your latest transactions</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {activities.map((activity) => (
            <div
              key={activity.id}
              className="flex items-center justify-between border-b border-border/40 pb-4 last:border-0 last:pb-0"
            >
              <div className="flex items-start gap-3">
                <div
                  className={`w-10 h-10 rounded-full flex items-center justify-center ${
                    activity.type === "deposit"
                      ? "bg-primary/10 text-primary"
                      : activity.type === "withdraw"
                        ? "bg-muted/30 text-muted-foreground"
                        : "bg-destructive/10 text-destructive"
                  }`}
                >
                  {activity.type === "deposit" ? "+" : "-"}
                </div>
                <div>
                  <p className="font-medium capitalize">{activity.type.replace("_", " ")}</p>
                  <p className="text-sm text-muted-foreground">
                    {activity.amount} • {activity.lockPeriod} lock
                  </p>
                </div>
              </div>
              <div className="text-right">
                <Badge
                  variant={
                    activity.status === "confirmed"
                      ? "outline"
                      : activity.status === "pending"
                        ? "secondary"
                        : "destructive"
                  }
                >
                  {activity.status}
                </Badge>
                <p className="text-xs text-muted-foreground mt-1">{formatDistanceToNow(activity.timestamp)}</p>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
