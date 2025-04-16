import Link from "next/link"
import Image from "next/image"
import { Button } from "@/components/ui/button"
import { OrbitalBackground } from "@/components/orbital-background"
import { LandingHeader } from "@/components/landing-header"
import { ArrowRight, Lock, Clock, Shield, Zap } from "lucide-react"

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col relative overflow-hidden">
      <OrbitalBackground density="high" />

      <LandingHeader />

      {/* Hero Section */}
      <section className="flex-1 flex flex-col items-center justify-center pt-20 pb-16 px-4">
        <div className="container max-w-6xl mx-auto text-center">
          <h1 className="text-4xl md:text-6xl lg:text-7xl font-bold tracking-tight mb-6">
            Bitcoin Time-Locked <span className="text-gradient">Deposits</span>
          </h1>
          <p className="text-xl md:text-2xl text-muted-foreground max-w-3xl mx-auto mb-8">
            Fast, Secure time-locked deposits on any device
          </p>
          <div className="flex flex-wrap gap-2 justify-center text-lg mb-12">
            <span className="text-muted-foreground">Bitcoin</span>
            <span className="text-muted-foreground">|</span>
            <span className="text-muted-foreground">Rune Tokens</span>
            <span className="text-muted-foreground">|</span>
            <span className="text-muted-foreground">Ordinals</span>
            <span className="text-muted-foreground">|</span>
            <span className="text-muted-foreground">Lightning Network</span>
          </div>

          <Link href="/dashboard" passHref>
            <Button size="lg" className="rounded-full px-8 py-6 text-lg group">
              Launch App <ArrowRight className="ml-2 h-5 w-5 group-hover:translate-x-1 transition-transform" />
            </Button>
          </Link>
        </div>
      </section>

      {/* App Screenshot */}
      <section className="py-16 px-4 relative">
        <div className="container max-w-6xl mx-auto">
          <div className="w-full max-w-5xl mx-auto overflow-hidden rounded-lg border border-primary/20 shadow-lg shadow-primary/10">
            <Image
              src="/images/btc.jpg"
              alt="BitLock Dashboard"
              width={1000}
              height={600}
              className="w-full h-auto"
            />
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 px-4">
        <div className="container max-w-6xl mx-auto">
          <h2 className="text-3xl md:text-4xl font-bold text-center mb-16">Key Features</h2>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            <div className="feature-card p-6 rounded-lg">
              <div className="feature-icon">
                <Lock className="h-8 w-8 text-primary" />
              </div>
              <h3 className="text-xl font-semibold mb-2">Secure Deposits</h3>
              <p className="text-muted-foreground">
                Securely lock your Bitcoin and other assets with military-grade encryption.
              </p>
            </div>

            <div className="feature-card p-6 rounded-lg">
              <div className="feature-icon">
                <Clock className="h-8 w-8 text-primary" />
              </div>
              <h3 className="text-xl font-semibold mb-2">Flexible Time Locks</h3>
              <p className="text-muted-foreground">
                Choose your lock period from 1 day to 1 year with customizable options.
              </p>
            </div>

            <div className="feature-card p-6 rounded-lg">
              <div className="feature-icon">
                <Shield className="h-8 w-8 text-primary" />
              </div>
              <h3 className="text-xl font-semibold mb-2">Emergency Access</h3>
              <p className="text-muted-foreground">Emergency withdrawal options with transparent fee structure.</p>
            </div>

            <div className="feature-card p-6 rounded-lg">
              <div className="feature-icon">
                <Zap className="h-8 w-8 text-primary" />
              </div>
              <h3 className="text-xl font-semibold mb-2">Multi-Asset Support</h3>
              <p className="text-muted-foreground">
                Support for Bitcoin, Rune tokens, Ordinals, and Lightning Network.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="py-16 px-4 border-t border-border/40">
        <div className="container max-w-6xl mx-auto">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            <div className="stats-item text-center">
              <p className="text-5xl md:text-6xl font-bold mb-2">$120M+</p>
              <p className="text-muted-foreground">Total Value Locked</p>
            </div>

            <div className="stats-item text-center">
              <p className="text-5xl md:text-6xl font-bold mb-2">24,500</p>
              <p className="text-muted-foreground">Unique Wallets</p>
            </div>

            <div className="stats-item text-center">
              <p className="text-5xl md:text-6xl font-bold mb-2">3,200</p>
              <p className="text-muted-foreground">Daily Active Users</p>
            </div>

            <div className="stats-item text-center">
              <p className="text-5xl md:text-6xl font-bold mb-2">7 Min</p>
              <p className="text-muted-foreground">Average Engagement</p>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="py-8 px-4 border-t border-border/40">
        <div className="container max-w-6xl mx-auto">
          <div className="flex flex-col md:flex-row justify-between items-center">
            <div className="flex items-center mb-4 md:mb-0">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="h-6 w-6 text-primary mr-2"
              >
                <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
              </svg>
              <span className="text-xl font-bold">BitLock</span>
            </div>

            <div className="flex gap-6">
              <Link href="/dashboard" className="text-sm text-muted-foreground hover:text-primary transition-colors">
                Dashboard
              </Link>
              <Link href="/docs" className="text-sm text-muted-foreground hover:text-primary transition-colors">
                Docs
              </Link>
              <Link href="/community" className="text-sm text-muted-foreground hover:text-primary transition-colors">
                Community
              </Link>
              <Link href="/governance" className="text-sm text-muted-foreground hover:text-primary transition-colors">
                Governance
              </Link>
            </div>
          </div>
          <div className="mt-8 text-center text-sm text-muted-foreground">
            © {new Date().getFullYear()} BitLock. All rights reserved.
          </div>
        </div>
      </footer>
    </div>
  )
}