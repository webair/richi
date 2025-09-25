<script lang="ts" setup>
import { authClient, withStandardizedError } from '@/shared/supabase'

const onLogout = async () => {
  await withStandardizedError(() => authClient.signOut())
}
</script>

<template>
  <form @submit.prevent="onLogout">
    <input type="submit" value="Abmelden" />
  </form>
</template>
