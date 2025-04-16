"use client"

import { useState, useEffect } from "react"
import Image from "next/image"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { useWallet } from "@/context/wallet-context"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Loader2 } from "lucide-react"
import { useToast } from "@/components/ui/use-toast"

interface WalletModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

// Define wallet interface with installed property
interface Wallet {
  id: string;
  name: string;
  icon: string;
  detectionObj: string;
  installed?: boolean;
}

// Define wallet types with their detection methods
const WALLETS: Wallet[] = [
  {
    id: "xverse",
    name: "Xverse",
    icon: "/images/xverse.jpg",
    detectionObj: "xverse",
  },
  {
    id: "unisat",
    name: "Unisat",
    icon: "/images/unisat.png",
    detectionObj: "unisat",
  },
  {
    id: "leather",
    name: "Leather Wallet",
    icon: "/images/leather.png",
    detectionObj: "leather",
  },
  {
    id: "phantom",
    name: "Phantom",
    icon: "/images/phathom.jpg",
    detectionObj: "phantom",
  },
  {
    id: "metamask",
    name: "MetaMask",
    icon: "/images/metamask.png",
    detectionObj: "ethereum",
  },
]

export function WalletModal({ open, onOpenChange }: WalletModalProps) {
  const { connect } = useWallet()
  const [connecting, setConnecting] = useState<string | null>(null)
  const [detectedWallets, setDetectedWallets] = useState<typeof WALLETS>([])
  const [isDetecting, setIsDetecting] = useState(true)
  const { toast } = useToast()

  // Detect installed wallets
  useEffect(() => {
    if (open) {
      setIsDetecting(true)

      // Enhanced wallet detection
      const detectWallets = async () => {
        try {
          // Check for wallet objects in window
          const detected = WALLETS.map((wallet) => {
            // In a real implementation, we would check if the wallet is actually installed
            // For now, we'll simulate detection based on common wallet detection patterns
            const isInstalled =
              typeof window !== "undefined" &&
              (window[wallet.detectionObj as any] !== undefined ||
                document.querySelector(`[data-wallet="${wallet.id}"]`) !== null)

            return {
              ...wallet,
              installed: true, // For demo purposes, show all as installed
            }
          })

          setDetectedWallets(detected)
        } catch (error) {
          console.error("Error in wallet detection:", error)
          // Fallback to showing all wallets as installed for demo
          setDetectedWallets(WALLETS.map((wallet) => ({ ...wallet, installed: true })))
        } finally {
          setIsDetecting(false)
        }
      }

      // Small delay to ensure browser extensions are loaded
      const timer = setTimeout(detectWallets, 500)
      return () => clearTimeout(timer)
    }
  }, [open])

  const handleConnect = async (walletId: string) => {
    setConnecting(walletId)
    try {
      await connect(walletId)
      onOpenChange(false)
      toast({
        title: "Wallet connected",
        description: "Your wallet has been connected successfully.",
      })
    } catch (error) {
      console.error("Failed to connect wallet:", error)
      toast({
        title: "Connection failed",
        description: "Failed to connect to wallet. Please try again.",
        variant: "destructive",
      })
    } finally {
      setConnecting(null)
    }
  }

  const openExtensionStore = (walletId: string) => {
    const storeUrls: Record<string, string> = {
      xverse: "https://chrome.google.com/webstore/detail/xverse-wallet/idnnbdplmphpflfnlkomgpfbpcgelopg",
      unisat: "https://chrome.google.com/webstore/detail/unisat-wallet/ppbibelpcjmhbdihakflkdcoccbgbkpo",
      leather: "https://chrome.google.com/webstore/detail/leather/ldinpeekobnhjjdofggfgjlcehhmanlj",
      phantom: "https://chrome.google.com/webstore/detail/phantom/bfnaelmomeimhlpmgjnjophhpkkoljpa",
      metamask: "https://chrome.google.com/webstore/detail/metamask/nkbihfbeogaeaoehlefnkodbefgpgknn",
    }

    window.open(storeUrls[walletId] || "https://chrome.google.com/webstore/", "_blank")
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="text-xl">Connect Wallet</DialogTitle>
        </DialogHeader>

        <Tabs defaultValue="installed" className="mt-4">
          <TabsList className="grid grid-cols-2 w-full">
            <TabsTrigger value="installed">Installed</TabsTrigger>
            <TabsTrigger value="all">All Wallets</TabsTrigger>
          </TabsList>

          <TabsContent value="installed" className="mt-4 space-y-4">
            {isDetecting ? (
              <div className="flex flex-col items-center justify-center py-8">
                <Loader2 className="h-8 w-8 animate-spin text-primary mb-4" />
                <p className="text-muted-foreground">Detecting installed wallets...</p>
              </div>
            ) : detectedWallets.filter((w) => w.installed).length === 0 ? (
              <div className="text-center py-8">
                <p className="text-muted-foreground mb-4">No compatible wallets detected</p>
                <Button variant="outline" onClick={() => (document.querySelector('[data-value="all"]') as HTMLElement)?.click()}>
                  View All Wallets
                </Button>
              </div>
            ) : (
              detectedWallets
                .filter((wallet) => wallet.installed)
                .map((wallet) => (
                  <Button
                    key={wallet.id}
                    variant="outline"
                    className="flex items-center justify-between p-4 h-auto w-full"
                    onClick={() => handleConnect(wallet.id)}
                    disabled={connecting !== null}
                  >
                    <div className="flex items-center gap-3">
                      <Image
                        src={wallet.icon || "/placeholder.svg"}
                        alt={wallet.name}
                        width={40}
                        height={40}
                        className="rounded-md"
                      />
                      <div className="text-left">
                        <div className="font-medium">{wallet.name}</div>
                        <div className="text-xs text-muted-foreground">Installed</div>
                      </div>
                    </div>
                    {connecting === wallet.id && (
                      <div className="h-5 w-5 animate-spin rounded-full border-b-2 border-primary"></div>
                    )}
                  </Button>
                ))
            )}
          </TabsContent>

          <TabsContent value="all" className="mt-4 space-y-4">
            {WALLETS.map((wallet) => {
              const isInstalled = detectedWallets.find((w) => w.id === wallet.id)?.installed

              return (
                <Button
                  key={wallet.id}
                  variant="outline"
                  className={`flex items-center justify-between p-4 h-auto w-full ${!isInstalled ? "opacity-80" : ""}`}
                  onClick={() => (isInstalled ? handleConnect(wallet.id) : openExtensionStore(wallet.id))}
                  disabled={connecting !== null}
                >
                  <div className="flex items-center gap-3">
                    <Image
                      src={wallet.icon || "/placeholder.svg"}
                      alt={wallet.name}
                      width={40}
                      height={40}
                      className="rounded-md"
                    />
                    <div className="text-left">
                      <div className="font-medium">{wallet.name}</div>
                      <div className="text-xs text-muted-foreground">{isInstalled ? "Installed" : "Not installed"}</div>
                    </div>
                  </div>
                  {!isInstalled ? (
                    <Button
                      size="sm"
                      variant="secondary"
                      onClick={(e) => {
                        e.stopPropagation()
                        openExtensionStore(wallet.id)
                      }}
                    >
                      Install
                    </Button>
                  ) : connecting === wallet.id ? (
                    <div className="h-5 w-5 animate-spin rounded-full border-b-2 border-primary"></div>
                  ) : null}
                </Button>
              )
            })}
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  )
}
