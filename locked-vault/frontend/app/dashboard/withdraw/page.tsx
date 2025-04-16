import { WithdrawList } from "@/components/withdraw-list"

export default function WithdrawPage() {
  return (
    <div className="container max-w-4xl mx-auto">
      <h1 className="text-3xl font-bold mb-6">Withdraw Funds</h1>
      <WithdrawList />
    </div>
  )
}