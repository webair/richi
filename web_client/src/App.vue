<script setup lang="ts">
import AppLayout from './app-layout/AppLayout.vue'
import LoginView from './login/LoginView.vue'
import OpenLockView from './open-lock/OpenLockView.vue'
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
  <AppLayout>
    <LoginView v-if="isUnauthenticatedState(state)" />

    <template v-if="isAuthenticatedState(state)">
      <OpenLockView :session="state.session" />
    </template>

    <p v-if="isErrorInitializingState(state)">
      Ein Fehler ist aufgetreten, bitte versuch es später nochmal
    </p>
    <p v-if="isInitializingState(state)">Initialisiere die App...</p>
  </AppLayout>
</template>
