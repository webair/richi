<script setup lang="ts">
import LoginView from './login/LoginView.vue'
import LogoutButton from './LogoutButton.vue'
import LockRequestView from './open-lock/OpenLockView.vue'
import {
  isAuthenticatedState,
  isErrorInitializingState,
  isInitializingState,
  isUnauthenticatedState,
  useAuth,
} from './shared/auth'

const { state } = useAuth()

</script>

<template>
  <h1>Richi</h1>

  <LoginView v-if="isUnauthenticatedState(state)" />

  <template v-if="isAuthenticatedState(state)">
    <LogoutButton />
    <LockRequestView :session="state.session" />
  </template>

  <p v-if="isErrorInitializingState(state)">
    Ein Fehler ist aufgetreten, bitte versuch es später nochmal
  </p>
  <p v-if="isInitializingState(state)">Initialisiere die App...</p>
</template>
