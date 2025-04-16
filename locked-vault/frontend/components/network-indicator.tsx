"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { Globe } from "lucide-react"
import { useToast } from "@/components/ui/use-toast"

export function NetworkIndicator() {
  const [network, setNetwork] = useState("mainnet")
  const { toast } = useToast()

  const handleNetworkChange = (newNetwork: string) => {
    setNetwork(newNetwork)
    toast({
      title: "Network changed",
      description: `Switched to ${newNetwork}`,
    })
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" className="hidden">
          <Globe size={14} />
          <span className="hidden">{network === "mainnet" ? "Mainnet" : "Testnet"}</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onClick={() => handleNetworkChange("testnet")}>Testnet</DropdownMenuItem>
        <DropdownMenuItem onClick={() => handleNetworkChange("mainnet")}>Mainnet</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
