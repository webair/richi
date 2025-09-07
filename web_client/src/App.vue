<script setup lang="ts">
import { AuthError, type Session } from '@supabase/supabase-js'
import { ref } from 'vue'

import { alwaysError } from './error'
import LoginView from './LoginView.vue'
import OpenDoorView from './OpenDoorView.vue'
import { authClient, withStandardizedError } from './supabase'

interface AuthenticatedState {
  type: 'authenticatedState'
  session: Session
}

interface InitializingAuthState {
  type: 'initializingAuthState'
}

interface AuthErrorState {
  type: 'authErrorState'
  error: Error
}

interface UnauthenticatedState {
  type: 'unauthenticatedState'
}

type AuthStatus = AuthenticatedState | InitializingAuthState | AuthErrorState | UnauthenticatedState

const state = ref<AuthStatus>({
  type: 'initializingAuthState',
})

const check = async () => {
  try {
    const { session } = await withStandardizedError(
      () =>
        authClient.getSession() as Promise<
          | { data: { session: Session | null }; error: null }
          | { data: { session: null }; error: AuthError }
        >, // cast to return type because otherwisee the generic T is not distinct
    )

    if (session) {
      state.value = { type: 'authenticatedState', session: session }
    } else {
      state.value = { type: 'unauthenticatedState' }
    }
  } catch (e: unknown) {
    const error = alwaysError(e)
    state.value = { type: 'authErrorState', error }
  }
}

check().then(() =>
  authClient.onAuthStateChange((_, session) => {
    if (session) {
      state.value = { type: 'authenticatedState', session: session }
    } else {
      state.value = { type: 'unauthenticatedState' }
    }
  }),
)
</script>

<template>
  <h1>Richi</h1>
  <LoginView v-if="state.type === 'unauthenticatedState'" />
  <OpenDoorView v-if="state.type === 'authenticatedState'" :session="state.session" />
  <p v-if="state.type === 'authErrorState'">
    Ein Fehler ist aufgetreten, bitte versuch es später nochmal
  </p>
  <p v-if="state.type === 'initializingAuthState'">Initialisiere die App...</p>
</template>
