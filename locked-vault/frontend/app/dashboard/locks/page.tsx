import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Progress } from "@/components/ui/progress"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { formatCurrency } from "@/lib/utils"

export default function LocksPage() {
  // Mock data
  const locks = [
    {
      id: "lock_1",
      asset: "Bitcoin",
      amount: "0.25",
      value: 15000,
      lockPeriod: 90,
      progress: 33,
      remainingDays: 60,
      status: "locked",
    },
    {
      id: "lock_2",
      asset: "Bitcoin",
      amount: "0.2",
      value: 12843.56,
      lockPeriod: 180,
      progress: 67,
      remainingDays: 60,
      status: "locked",
    },
    {
      id: "lock_3",
      asset: "Rune",
      amount: "500",
      value: 15000,
      lockPeriod: 30,
      progress: 50,
      remainingDays: 15,
      status: "locked",
    },
    {
      id: "lock_4",
      asset: "Ordinals",
      amount: "1",
      value: 8000,
      lockPeriod: 365,
      progress: 25,
      remainingDays: 274,
      status: "locked",
    },
    {
      id: "lock_5",
      asset: "Bitcoin",
      amount: "0.1",
      value: 6000,
      lockPeriod: 30,
      progress: 100,
      remainingDays: 0,
      status: "unlocked",
    },
  ]

  return (
    <div className="container max-w-6xl mx-auto">
      <h1 className="text-3xl font-bold mb-6">Time Locks</h1>

      <Tabs defaultValue="active" className="space-y-6">
        <TabsList className="grid grid-cols-3 w-full max-w-md">
          <TabsTrigger value="active">Active Locks</TabsTrigger>
          <TabsTrigger value="completed">Completed</TabsTrigger>
          <TabsTrigger value="all">All Locks</TabsTrigger>
        </TabsList>

        <TabsContent value="active" className="space-y-6">
          <Card className="glow-card card-gradient">
            <CardHeader>
              <CardTitle>Active Time Locks</CardTitle>
              <CardDescription>Your current time-locked deposits</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-6">
                {locks
                  .filter((lock) => lock.status === "locked")
                  .map((lock) => (
                    <div key={lock.id} className="border-b border-border pb-6 last:border-0 last:pb-0">
                      <div className="flex justify-between items-start mb-4">
                        <div>
                          <h3 className="text-xl font-bold">
                            {lock.amount} {lock.asset}
                          </h3>
                          <p className="text-muted-foreground">
                            ${formatCurrency(lock.value)} • {lock.lockPeriod} day lock
                          </p>
                        </div>
                        <Badge variant={lock.status === "locked" ? "secondary" : "outline"}>{lock.status}</Badge>
                      </div>

                      <div className="space-y-2">
                        <div className="flex justify-between text-sm">
                          <span>Lock progress</span>
                          <span>{lock.progress}%</span>
                        </div>
                        <Progress value={lock.progress} className="h-2" />
                        <div className="flex justify-between text-xs text-muted-foreground">
                          <span>0 days</span>
                          <span>{lock.lockPeriod} days</span>
                        </div>
                      </div>

                      <div className="mt-4 flex items-center justify-between">
                        <div className="text-sm">
                          <span className="text-muted-foreground">Time remaining: </span>
                          <span className="font-medium">{lock.remainingDays} days</span>
                        </div>
                        <Button variant="outline" size="sm" disabled={lock.status !== "unlocked"}>
                          {lock.status === "unlocked" ? "Withdraw" : "Locked"}
                        </Button>
                      </div>
                    </div>
                  ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="completed" className="space-y-6">
          <Card className="glow-card card-gradient">
            <CardHeader>
              <CardTitle>Completed Time Locks</CardTitle>
              <CardDescription>Your unlocked deposits ready for withdrawal</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-6">
                {locks
                  .filter((lock) => lock.status === "unlocked")
                  .map((lock) => (
                    <div key={lock.id} className="border-b border-border pb-6 last:border-0 last:pb-0">
                      <div className="flex justify-between items-start mb-4">
                        <div>
                          <h3 className="text-xl font-bold">
                            {lock.amount} {lock.asset}
                          </h3>
                          <p className="text-muted-foreground">
                            ${formatCurrency(lock.value)} • {lock.lockPeriod} day lock
                          </p>
                        </div>
                        <Badge variant="outline">unlocked</Badge>
                      </div>

                      <div className="space-y-2">
                        <div className="flex justify-between text-sm">
                          <span>Lock progress</span>
                          <span>100%</span>
                        </div>
                        <Progress value={100} className="h-2" />
                      </div>

                      <div className="mt-4 flex items-center justify-between">
                        <div className="text-sm">
                          <span className="text-muted-foreground">Status: </span>
                          <span className="font-medium text-success">Ready for withdrawal</span>
                        </div>
                        <Button size="sm">Withdraw</Button>
                      </div>
                    </div>
                  ))}

                {locks.filter((lock) => lock.status === "unlocked").length === 0 && (
                  <div className="text-center py-8">
                    <p className="text-muted-foreground">You don't have any completed time locks yet.</p>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="all" className="space-y-6">
          <Card className="glow-card card-gradient">
            <CardHeader>
              <CardTitle>All Time Locks</CardTitle>
              <CardDescription>Complete history of your time-locked deposits</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-6">
                {locks.map((lock) => (
                  <div key={lock.id} className="border-b border-border pb-6 last:border-0 last:pb-0">
                    <div className="flex justify-between items-start mb-4">
                      <div>
                        <h3 className="text-xl font-bold">
                          {lock.amount} {lock.asset}
                        </h3>
                        <p className="text-muted-foreground">
                          ${formatCurrency(lock.value)} • {lock.lockPeriod} day lock
                        </p>
                      </div>
                      <Badge variant={lock.status === "locked" ? "secondary" : "outline"}>{lock.status}</Badge>
                    </div>

                    <div className="space-y-2">
                      <div className="flex justify-between text-sm">
                        <span>Lock progress</span>
                        <span>{lock.progress}%</span>
                      </div>
                      <Progress value={lock.progress} className="h-2" />
                    </div>

                    <div className="mt-4 flex items-center justify-between">
                      <div className="text-sm">
                        {lock.status === "locked" ? (
                          <>
                            <span className="text-muted-foreground">Time remaining: </span>
                            <span className="font-medium">{lock.remainingDays} days</span>
                          </>
                        ) : (
                          <span className="font-medium text-success">Ready for withdrawal</span>
                        )}
                      </div>
                      <Button
                        variant={lock.status === "unlocked" ? "default" : "outline"}
                        size="sm"
                        disabled={lock.status !== "unlocked"}
                      >
                        {lock.status === "unlocked" ? "Withdraw" : "Locked"}
                      </Button>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}