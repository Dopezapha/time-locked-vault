import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { OrbitalBackground } from "@/components/orbital-background"

export default function DocsPage() {
  return (
    <div className="min-h-screen flex flex-col relative">
      <OrbitalBackground density="low" />

      <header className="border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container flex h-16 items-center">
          <div className="flex items-center gap-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="h-6 w-6 text-primary"
            >
              <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
            </svg>
            <span className="text-xl font-bold">BitLock Documentation</span>
          </div>
        </div>
      </header>

      <main className="flex-1 container py-8">
        <div className="max-w-5xl mx-auto">
          <h1 className="text-4xl font-bold mb-6">Documentation</h1>

          <Tabs defaultValue="overview" className="space-y-8">
            <TabsList className="grid grid-cols-4 w-full max-w-lg">
              <TabsTrigger value="overview">Overview</TabsTrigger>
              <TabsTrigger value="guides">Guides</TabsTrigger>
              <TabsTrigger value="api">API</TabsTrigger>
              <TabsTrigger value="contracts">Contracts</TabsTrigger>
            </TabsList>

            <TabsContent value="overview" className="space-y-6">
              <Card>
                <CardHeader>
                  <CardTitle>What is BitLock?</CardTitle>
                  <CardDescription>A secure time-locked deposit system for Bitcoin</CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <p>
                    BitLock is a decentralized application that allows users to create time-locked deposits for Bitcoin,
                    Rune tokens, Ordinals, and Lightning Network payments. With BitLock, you can securely lock your
                    assets for a specified period of time, ensuring they cannot be accessed until the lock period
                    expires.
                  </p>
                  <p>
                    Our platform provides a user-friendly interface for managing your time-locked deposits, with
                    features such as customizable lock periods, emergency withdrawal options, and a comprehensive
                    portfolio view.
                  </p>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Key Features</CardTitle>
                </CardHeader>
                <CardContent>
                  <ul className="list-disc pl-6 space-y-2">
                    <li>Secure time-locked deposits for Bitcoin and other assets</li>
                    <li>Customizable lock periods from 1 day to 1 year</li>
                    <li>Emergency withdrawal options with transparent fee structure</li>
                    <li>Support for Bitcoin, Rune tokens, Ordinals, and Lightning Network</li>
                    <li>Comprehensive portfolio management</li>
                    <li>User-friendly interface</li>
                  </ul>
                </CardContent>
              </Card>
            </TabsContent>

            <TabsContent value="guides" className="space-y-6">
              <Card>
                <CardHeader>
                  <CardTitle>Getting Started</CardTitle>
                  <CardDescription>Learn how to use BitLock</CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <h3 className="text-lg font-medium">1. Connect Your Wallet</h3>
                  <p>
                    To use BitLock, you need to connect a compatible Bitcoin wallet. We support Xverse, Unisat, Leather
                    Wallet, Phantom, and MetaMask. Click the "Connect Wallet" button in the top right corner to get
                    started.
                  </p>

                  <h3 className="text-lg font-medium">2. Create a Time-Locked Deposit</h3>
                  <p>
                    Navigate to the Deposit page and select the token type you want to lock. Enter the amount and choose
                    your desired lock period. Review the details and confirm the transaction.
                  </p>

                  <h3 className="text-lg font-medium">3. Monitor Your Portfolio</h3>
                  <p>
                    Use the Portfolio page to track all your time-locked deposits and their status. You can view
                    detailed information about each deposit, including the unlock date and current value.
                  </p>

                  <h3 className="text-lg font-medium">4. Withdraw Funds</h3>
                  <p>
                    Once a lock period expires, you can withdraw your funds from the Withdraw page. If you need to
                    access your funds before the lock period ends, you can use the emergency withdrawal option, which
                    incurs a 10% penalty fee.
                  </p>
                </CardContent>
              </Card>
            </TabsContent>

            <TabsContent value="api" className="space-y-6">
              <Card>
                <CardHeader>
                  <CardTitle>API Reference</CardTitle>
                  <CardDescription>Integrate with BitLock programmatically</CardDescription>
                </CardHeader>
                <CardContent>
                  <p className="mb-4">
                    BitLock provides a RESTful API for developers who want to integrate with our platform. The API
                    allows you to create and manage time-locked deposits programmatically.
                  </p>

                  <div className="space-y-4">
                    <div className="p-4 border rounded-md">
                      <h3 className="text-lg font-medium">GET /api/deposits</h3>
                      <p className="text-sm text-muted-foreground">Retrieve all deposits for the authenticated user</p>
                    </div>

                    <div className="p-4 border rounded-md">
                      <h3 className="text-lg font-medium">POST /api/deposits</h3>
                      <p className="text-sm text-muted-foreground">Create a new time-locked deposit</p>
                    </div>

                    <div className="p-4 border rounded-md">
                      <h3 className="text-lg font-medium">GET /api/deposits/{"{id}"}</h3>
                      <p className="text-sm text-muted-foreground">Retrieve details for a specific deposit</p>
                    </div>

                    <div className="p-4 border rounded-md">
                      <h3 className="text-lg font-medium">POST /api/deposits/{"{id}"}/withdraw</h3>
                      <p className="text-sm text-muted-foreground">Withdraw funds from a deposit</p>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>

            <TabsContent value="contracts" className="space-y-6">
              <Card>
                <CardHeader>
                  <CardTitle>Smart Contracts</CardTitle>
                  <CardDescription>Technical details of the BitLock contracts</CardDescription>
                </CardHeader>
                <CardContent>
                  <p className="mb-4">
                    BitLock uses smart contracts to secure your time-locked deposits. Our contracts are open-source and
                    have been audited by leading security firms.
                  </p>

                  <div className="space-y-4">
                    <div className="p-4 border rounded-md">
                      <h3 className="text-lg font-medium">BitLockDeposit.sol</h3>
                      <p className="text-sm text-muted-foreground mb-2">
                        The main contract for managing time-locked deposits
                      </p>
                      <p className="text-xs font-mono bg-muted p-2 rounded">Contract Address: 0x1234...5678</p>
                    </div>

                    <div className="p-4 border rounded-md">
                      <h3 className="text-lg font-medium">BitLockToken.sol</h3>
                      <p className="text-sm text-muted-foreground mb-2">ERC-20 token contract for BitLock governance</p>
                      <p className="text-xs font-mono bg-muted p-2 rounded">Contract Address: 0xabcd...ef01</p>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
        </div>
      </main>
    </div>
  )
}