import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/something')({
  component: () => <div>Hello /something!</div>
})