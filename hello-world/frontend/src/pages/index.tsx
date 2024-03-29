import { Route, Routes } from 'react-router-dom'
import { useTamagotchiInit } from '@/app/hooks/use-tamagotchi'
import { useThrottleWasmState } from '@/app/hooks/use-read-wasm-state'
import { useItemsStore } from '@/app/hooks/use-ft-store'
import { lazy, Suspense } from 'react'

const routes = [
  { path: '/', Page: lazy(() => import('./home')) },
  { path: '/store', Page: lazy(() => import('./store')) },
  { path: '/battle', Page: lazy(() => import('./battle')) },
]

export const Routing = () => {
  useTamagotchiInit()
  useThrottleWasmState()
  useItemsStore()

  return (
    <Routes>
      {routes.map(({ path, Page }) => (
        <Route
          key={path}
          path={path}
          element={
            <Suspense fallback={<>Page {Page.name} is loading...</>}>
              <Page />
            </Suspense>
          }
        />
      ))}
    </Routes>
  )
}
