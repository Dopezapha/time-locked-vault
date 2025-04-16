import { DepositForm } from "@/components/deposit-form"

export default function DepositPage() {
  return (
    <div className="container max-w-2xl mx-auto">
      <h1 className="text-3xl font-bold mb-6">Create Time-Locked Deposit</h1>
      <DepositForm />
    </div>
  )
}