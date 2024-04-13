import type { ReactNode } from 'react'

export function Container({ children }: { children: ReactNode }) {
  return (
    <div className="flex flex-col min-h-screen h-full w-full overflow-y-auto">
      {children}
    </div>
  )
}
