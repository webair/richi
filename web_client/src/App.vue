<script setup lang="ts">
import { AuthError, type Session } from '@supabase/supabase-js'
import { ref } from 'vue'

import { alwaysError } from './error'
import LockRequestView from './LockRequestView.vue'
import LoginView from './LoginView.vue'
import { authClient, withStandardizedError } from './supabase'

interface AuthenticatedState {
  type: 'authenticatedState'
  session: Session
}

interface InitializingState {
  type: 'initializingState'
}

interface ErrorInitializingState {
  type: 'errorInitializingState'
  error: Error
}

interface UnauthenticatedState {
  type: 'unauthenticatedState'
}

type AuthenticationState =
  | AuthenticatedState
  | InitializingState
  | ErrorInitializingState
  | UnauthenticatedState

const state = ref<AuthenticationState>({
  type: 'initializingState',
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
    state.value = { type: 'errorInitializingState', error }
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

const onLogout = async () => {
  await withStandardizedError(() => authClient.signOut())
}
</script>

<template>
  <h1>Richi</h1>

  <LoginView v-if="state.type === 'unauthenticatedState'" />

  <template v-if="state.type === 'authenticatedState'">
    <form @submit.prevent="onLogout">
      <input type="submit" value="Abmelden" />
    </form>
    <LockRequestView :session="state.session" />
  </template>

  <p v-if="state.type === 'errorInitializingState'">
    Ein Fehler ist aufgetreten, bitte versuch es später nochmal
  </p>
  <p v-if="state.type === 'initializingState'">Initialisiere die App...</p>
</template>
