<script lang="ts" setup>
import { NButton } from 'naive-ui'
import { ref } from 'vue'

import { useAuthClient } from '@/shared/auth'

const authClient = useAuthClient()

const loggingOut = ref(false)

const onLogout = async () => {
  loggingOut.value = true
  await authClient.logout()
}
</script>

<template>
  <form @submit.prevent="onLogout">
    <NButton
      type="tertiary"
      attr-type="submit"
      size="large"
      :disabled="loggingOut"
      :loading="loggingOut"
      >Abmelden</NButton
    >
  </form>
</template>
