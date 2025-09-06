<script lang="ts" setup>
import { ref } from 'vue'

import { authClient, withStandardizedError } from './supabase'

const loggingOut = ref(false)

const onLogout = async () => {
  await withStandardizedError(() => authClient.signOut())
}
</script>

<template>
  <form @submit.prevent="onLogout">
    <input type="submit" value="Abmelden" :disabled="loggingOut" />
  </form>
</template>
>
