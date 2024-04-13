import { Outlet, createRootRouteWithContext } from '@tanstack/react-router'

import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import type { useAuth } from '@clerk/clerk-react'
import { Container } from '~/components/Container'
import { Navbar } from '~/components/Navbar'

export type RouterContext = {
  auth: ReturnType<typeof useAuth>
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: RootComponent,
})

function RootComponent() {
  return (
    <>
      <Container>
        <Navbar />
        <Outlet />
      </Container>
      <TanStackRouterDevtools position="bottom-right" />
    </>
  )
}
