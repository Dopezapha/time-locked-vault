"use client"

import { useEffect, useRef } from "react"

interface OrbitalBackgroundProps {
  className?: string
  density?: "low" | "medium" | "high"
  animated?: boolean
}

export function OrbitalBackground({ className = "", density = "medium", animated = true }: OrbitalBackgroundProps) {
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const container = containerRef.current
    if (!container) return

    // Clear any existing dots
    const existingCircles = container.querySelectorAll(".orbital-circle")
    existingCircles.forEach((circle) => {
      const dots = circle.querySelectorAll(".orbital-dot")
      dots.forEach((dot) => dot.remove())
    })

    // Create orbital dots
    const createDots = (circleClass: string, count: number) => {
      const circle = container.querySelector(`.${circleClass}`) as HTMLElement
      if (!circle) return

      const radius = Number.parseInt(getComputedStyle(circle).width) / 2

      for (let i = 0; i < count; i++) {
        const angle = (i / count) * 2 * Math.PI
        const x = radius * Math.cos(angle)
        const y = radius * Math.sin(angle)

        const dot = document.createElement("div")
        dot.className = "orbital-dot"
        dot.style.left = `calc(50% + ${x}px)`
        dot.style.top = `calc(50% + ${y}px)`

        // Add random animation delay for twinkling effect
        if (animated) {
          dot.style.animation = `pulse-glow ${3 + Math.random() * 2}s ease-in-out infinite ${Math.random() * 2}s`
        }

        circle.appendChild(dot)
      }
    }

    // Set dot counts based on density
    const dotCounts = {
      low: [6, 8, 12],
      medium: [8, 12, 16],
      high: [12, 18, 24],
    }

    const [count1, count2, count3] = dotCounts[density]
    createDots("orbital-circle-1", count1)
    createDots("orbital-circle-2", count2)
    createDots("orbital-circle-3", count3)

    return () => {
      const circles = container.querySelectorAll(".orbital-circle")
      circles.forEach((circle) => {
        const dots = circle.querySelectorAll(".orbital-dot")
        dots.forEach((dot) => dot.remove())
      })
    }
  }, [density, animated])

  return (
    <div ref={containerRef} className={`orbital-bg ${className}`}>
      <div className="orbital-circle orbital-circle-1"></div>
      <div className="orbital-circle orbital-circle-2"></div>
      <div className="orbital-circle orbital-circle-3"></div>
    </div>
  )
}
