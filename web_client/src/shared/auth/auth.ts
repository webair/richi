import { type Ref, ref } from 'vue'

import { alwaysError } from '@/shared/error'

import { useAuthClient } from './auth-client'
import type { Session } from './index'

interface AuthenticatedState {
  type: 'authenticated'
  session: Session
}

interface InitializingState {
  type: 'initializing'
}

interface ErrorInitializingState {
  type: 'errorInitializing'
  error: Error
}

interface UnauthenticatedState {
  type: 'unauthenticated'
}

type AuthState =
  | AuthenticatedState
  | InitializingState
  | ErrorInitializingState
  | UnauthenticatedState

export const isAuthenticatedState = (authState: AuthState): authState is AuthenticatedState => {
  return authState.type === 'authenticated'
}

export const isInitializingState = (authState: AuthState): authState is InitializingState => {
  return authState.type === 'initializing'
}

export const isErrorInitializingState = (
  authState: AuthState
): authState is ErrorInitializingState => {
  return authState.type === 'errorInitializing'
}

export const isUnauthenticatedState = (authState: AuthState): authState is UnauthenticatedState => {
  return authState.type === 'unauthenticated'
}

interface Auth {
  state: Ref<AuthState>
}

const createAuth = (): Auth => {
  const authClient = useAuthClient()
  const state = ref<AuthState>({
    type: 'initializing',
  })

  const initializeState = async () => {
    try {
      const session = await authClient.getSession()
      if (session) {
        state.value = { type: 'authenticated', session: session }
      } else {
        state.value = { type: 'unauthenticated' }
      }
    } catch (e: unknown) {
      state.value = { type: 'errorInitializing', error: alwaysError(e) }
    }
  }

  initializeState().then(() =>
    authClient.onAuthStateChange((_, session) => {
      if (session) {
        state.value = { type: 'authenticated', session: session }
      } else {
        state.value = { type: 'unauthenticated' }
      }
    })
  )

  return {
    state,
  }
}

let auth: Auth | null = null

export const useAuth = (): Auth => {
  if (!auth) {
    auth = createAuth()
  }
  return auth
}
