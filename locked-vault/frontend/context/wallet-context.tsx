"use client"

import { createContext, useContext, useState, useEffect, type ReactNode } from "react"
import { useToast } from "@/components/ui/use-toast"

interface WalletContextType {
  connected: boolean
  address: string | null
  balance: number
  connect: (walletId: string) => Promise<void>
  disconnect: () => void
}

const WalletContext = createContext<WalletContextType>({
  connected: false,
  address: null,
  balance: 0,
  connect: async () => {},
  disconnect: () => {},
})

export const useWallet = () => useContext(WalletContext)

interface WalletProviderProps {
  children: ReactNode
}

export function WalletProvider({ children }: WalletProviderProps) {
  const [connected, setConnected] = useState(false)
  const [address, setAddress] = useState<string | null>(null)
  const [balance, setBalance] = useState(0)
  const [walletType, setWalletType] = useState<string | null>(null)
  const { toast } = useToast()

  // Check for previously connected wallet in localStorage
  useEffect(() => {
    const savedWallet = localStorage.getItem("connectedWallet")
    if (savedWallet) {
      try {
        const walletData = JSON.parse(savedWallet)
        setConnected(true)
        setAddress(walletData.address)
        setBalance(walletData.balance)
        setWalletType(walletData.type)
      } catch (error) {
        console.error("Failed to restore wallet connection:", error)
        localStorage.removeItem("connectedWallet")
      }
    }
  }, [])

  const connect = async (walletId: string) => {
    // Simulate wallet connection with different addresses based on wallet type
    return new Promise<void>((resolve, reject) => {
      try {
        // For demo purposes, always succeed in connecting
        // In a real implementation, you would check if the wallet is available

        // Simulate connection delay
        setTimeout(() => {
          // Generate mock data based on wallet type
          const mockData = {
            xverse: {
              address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
              balance: 0.12345,
            },
            unisat: {
              address: "bc1p3xqwzmddceqrd6x9yxplqzkl5vucta4nz5x9qkwtwn8q2rndxmzqxuzcqe",
              balance: 0.23456,
            },
            leather: {
              address: "3FZbgi29cpjq2GjdwV8eyHuJJnkLtktZc5",
              balance: 0.34567,
            },
            phantom: {
              address: "bc1qm34lsc65zpw79lxes69zkqmk6ee3ewf0j77s3h",
              balance: 0.45678,
            },
            metamask: {
              address: "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq",
              balance: 0.56789,
            },
          }[walletId] || { address: "bc1default", balance: 0.11111 }

          setConnected(true)
          setAddress(mockData.address)
          setBalance(mockData.balance)
          setWalletType(walletId)

          // Save connection to localStorage
          localStorage.setItem(
            "connectedWallet",
            JSON.stringify({
              address: mockData.address,
              balance: mockData.balance,
              type: walletId,
            }),
          )

          resolve()
        }, 1000)
      } catch (error) {
        reject(error)
      }
    })
  }

  const disconnect = () => {
    setConnected(false)
    setAddress(null)
    setBalance(0)
    setWalletType(null)
    localStorage.removeItem("connectedWallet")
  }

  return (
    <WalletContext.Provider value={{ connected, address, balance, connect, disconnect }}>
      {children}
    </WalletContext.Provider>
  )
}
