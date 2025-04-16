"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Wallet } from "lucide-react"
import { useWallet } from "@/context/wallet-context"
import { WalletModal } from "@/components/wallet-modal"
import { useToast } from "@/components/ui/use-toast"

export function WalletButton() {
  const { connected, address, disconnect } = useWallet()
  const [showModal, setShowModal] = useState(false)
  const { toast } = useToast()

  const formatAddress = (addr: string) => {
    return `${addr.substring(0, 6)}...${addr.substring(addr.length - 4)}`
  }

  const handleClick = () => {
    if (connected) {
      disconnect()
      toast({
        title: "Wallet disconnected",
        description: "Your wallet has been disconnected successfully.",
      })
    } else {
      setShowModal(true)
    }
  }

  return (
    <>
      <Button
        onClick={handleClick}
        className="gap-2 rounded-full text-sm sm:text-base"
        variant={connected ? "outline" : "default"}
        size={window.innerWidth < 640 ? "sm" : "default"}
      >
        <Wallet size={16} />
        {connected ? formatAddress(address || "") : "Connect Wallet"}
      </Button>

      <WalletModal open={showModal} onOpenChange={setShowModal} />
    </>
  )
}
