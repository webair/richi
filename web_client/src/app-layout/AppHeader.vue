<script lang="ts" setup>
import { isAuthenticatedState, useAuth } from '@/shared/auth'

import LogoutButton from './LogoutButton.vue'

const { state: authState } = useAuth()
</script>

<template>
  <div class="app-header">
    <h1>Richi</h1>
    <LogoutButton v-if="isAuthenticatedState(authState)" />
  </div>
</template>

<style>
.app-header {
  padding: var(--spacing-default) 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--color-divider);
}
</style>
