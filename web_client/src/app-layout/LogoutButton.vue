<script lang="ts" setup>
import { useAuthClient } from '@/shared/auth'

const authClient = useAuthClient()

const onLogout = async () => authClient.logout()
</script>

<template>
  <form @submit.prevent="onLogout">
    <input type="submit" value="Abmelden" />
  </form>
</template>
