/* prettier-ignore-start */

/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file is auto-generated by TanStack Router

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as WstestImport } from './routes/ws_test'
import { Route as UploadImport } from './routes/upload'
import { Route as TestImport } from './routes/test'
import { Route as SubImport } from './routes/sub'
import { Route as PubsubtestImport } from './routes/pubsub_test'
import { Route as AuthImport } from './routes/_auth'
import { Route as IndexImport } from './routes/index'
import { Route as AuthProfileImport } from './routes/_auth/profile'
import { Route as AuthChatImport } from './routes/_auth/chat'

// Create/Update Routes

const WstestRoute = WstestImport.update({
  path: '/ws_test',
  getParentRoute: () => rootRoute,
} as any)

const UploadRoute = UploadImport.update({
  path: '/upload',
  getParentRoute: () => rootRoute,
} as any)

const TestRoute = TestImport.update({
  path: '/test',
  getParentRoute: () => rootRoute,
} as any)

const SubRoute = SubImport.update({
  path: '/sub',
  getParentRoute: () => rootRoute,
} as any)

const PubsubtestRoute = PubsubtestImport.update({
  path: '/pubsub_test',
  getParentRoute: () => rootRoute,
} as any)

const AuthRoute = AuthImport.update({
  id: '/_auth',
  getParentRoute: () => rootRoute,
} as any)

const IndexRoute = IndexImport.update({
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

const AuthProfileRoute = AuthProfileImport.update({
  path: '/profile',
  getParentRoute: () => AuthRoute,
} as any)

const AuthChatRoute = AuthChatImport.update({
  path: '/chat',
  getParentRoute: () => AuthRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      preLoaderRoute: typeof IndexImport
      parentRoute: typeof rootRoute
    }
    '/_auth': {
      preLoaderRoute: typeof AuthImport
      parentRoute: typeof rootRoute
    }
    '/pubsub_test': {
      preLoaderRoute: typeof PubsubtestImport
      parentRoute: typeof rootRoute
    }
    '/sub': {
      preLoaderRoute: typeof SubImport
      parentRoute: typeof rootRoute
    }
    '/test': {
      preLoaderRoute: typeof TestImport
      parentRoute: typeof rootRoute
    }
    '/upload': {
      preLoaderRoute: typeof UploadImport
      parentRoute: typeof rootRoute
    }
    '/ws_test': {
      preLoaderRoute: typeof WstestImport
      parentRoute: typeof rootRoute
    }
    '/_auth/chat': {
      preLoaderRoute: typeof AuthChatImport
      parentRoute: typeof AuthImport
    }
    '/_auth/profile': {
      preLoaderRoute: typeof AuthProfileImport
      parentRoute: typeof AuthImport
    }
  }
}

// Create and export the route tree

export const routeTree = rootRoute.addChildren([
  IndexRoute,
  AuthRoute.addChildren([AuthChatRoute, AuthProfileRoute]),
  PubsubtestRoute,
  SubRoute,
  TestRoute,
  UploadRoute,
  WstestRoute,
])

/* prettier-ignore-end */
