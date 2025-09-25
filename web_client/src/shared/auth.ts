import type { AuthError, Session } from '@supabase/supabase-js'
import type { Plugin } from 'vue'
import { inject, type InjectionKey, type Ref, ref } from 'vue'

import { alwaysError } from '@/shared/error'
import { authClient, withStandardizedError } from '@/supabase'

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
  loginWithPhoneNumber: (phoneNumber: string) => Promise<void>
  verifyPhoneNumber: (phoneNumber: string, verificationCode: string) => Promise<void>
}

const createAuth = (): Auth => {
  const state = ref<AuthState>({
    type: 'initializing',
  })

  const initializeState = async () => {
    try {
      const { session } = await withStandardizedError(
        () =>
          authClient.getSession() as Promise<
            | { data: { session: Session | null }; error: null }
            | { data: { session: null }; error: AuthError }
          >
        // cast to return type because otherwisee the generic T is not distinct. This is a
        // workaround for a known upstream typing issue and should be removed if the underlying
        // types are fixed in the future.
      )

      if (session) {
        state.value = { type: 'authenticated', session: session }
      } else {
        state.value = { type: 'unauthenticated' }
      }
    } catch (e: unknown) {
      const error = alwaysError(e)
      state.value = { type: 'errorInitializing', error }
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

  const loginWithPhoneNumber = async (phoneNumber: string) => {
    await withStandardizedError(() =>
      authClient.signInWithOtp({
        phone: phoneNumber,
        options: { channel: 'sms' },
      })
    )
  }

  const verifyPhoneNumber = async (phoneNumber: string, verificationCode: string) => {
    await withStandardizedError(() =>
      authClient.verifyOtp({
        token: verificationCode,
        phone: phoneNumber,
        type: 'sms',
      })
    )
  }

  return {
    state,
    loginWithPhoneNumber,
    verifyPhoneNumber,
  }
}

const AuthKey: InjectionKey<Auth> = Symbol('authKey')

export const useAuth = (): Auth => {
  const auth = inject(AuthKey)
  if (!auth) {
    throw new Error('Auth not provided. Please ensure AuthPlugin is installed')
  }
  return auth
}

export const AuthPlugin: Plugin = {
  install: (app) => {
    const auth = createAuth()
    app.provide(AuthKey, auth)
  },
}
