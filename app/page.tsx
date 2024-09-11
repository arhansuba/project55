import Image from 'next/image'
import { Button } from "@/components/ui/button"
import { ThumbsUp, ThumbsDown } from 'lucide-react'

export default function Component() {
  return (
    <div className="min-h-screen bg-black text-white">
      <header className="container mx-auto px-4 py-6 flex justify-between items-center">
        <div className="w-24 h-24 relative">
          <Image
            src="/images/logo.png"
            alt="CITY FLOW Logo"
            fill
            className="object-contain"
          />
        </div>
        <Button 
          className="bg-black text-white border border-white hover:bg-white hover:text-black transition-colors"
        >
          Connect Wallet
        </Button>
      </header>
      <main className="container mx-auto px-4 py-8">
        <div className="max-w-2xl mx-auto bg-gray-900 rounded-lg overflow-hidden shadow-xl">
          <div className="p-4 border-b border-gray-800 flex items-center space-x-3">
            <div className="w-10 h-10 rounded-full bg-gray-700" />
            <span className="font-semibold">Username</span>
          </div>
          <div className="aspect-square relative">
            <Image
              src="/placeholder.svg?height=600&width=600"
              alt="Post image"
              fill
              className="object-cover"
            />
          </div>
          <div className="p-4 space-y-3">
            <div className="flex justify-between items-center">
              <div className="flex space-x-4">
                <button className="flex items-center space-x-1 text-green-500 hover:text-green-400 transition-colors">
                  <ThumbsUp className="h-6 w-6" />
                  <span>0</span>
                </button>
                <button className="flex items-center space-x-1 text-red-500 hover:text-red-400 transition-colors">
                  <ThumbsDown className="h-6 w-6" />
                  <span>0</span>
                </button>
              </div>
              <span className="text-sm text-gray-400">Vote Score: 0</span>
            </div>
            <p><span className="font-semibold">Username</span> This is a sample post caption. It can be much longer and may include hashtags and mentions.</p>
            <p className="text-gray-400 text-xs">2 HOURS AGO</p>
          </div>
        </div>
      </main>
    </div>
  )
}