<script setup lang="ts">
import LockRequestView from './LockRequestView.vue'
import LoginView from './login/LoginView.vue'
import {
  isAuthenticatedState,
  isErrorInitializingState,
  isInitializingState,
  isUnauthenticatedState,
  useAuth,
} from './shared/auth'
import { authClient, withStandardizedError } from './supabase'

const { state } = useAuth()

const onLogout = async () => {
  await withStandardizedError(() => authClient.signOut())
}
</script>

<template>
  <h1>Richi</h1>

  <LoginView v-if="isUnauthenticatedState(state)" />

  <template v-if="isAuthenticatedState(state)">
    <form @submit.prevent="onLogout">
      <input type="submit" value="Abmelden" />
    </form>
    <LockRequestView :session="state.session" />
  </template>

  <p v-if="isErrorInitializingState(state)">
    Ein Fehler ist aufgetreten, bitte versuch es später nochmal
  </p>
  <p v-if="isInitializingState(state)">Initialisiere die App...</p>
</template>
