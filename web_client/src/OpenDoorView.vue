<script lang="ts" setup>
import type { Session } from '@supabase/supabase-js'
import { ref } from 'vue'

import { authClient, withStandardizedError } from './supabase'

const props = defineProps<{ session: Session }>()

const loggingOut = ref(false)
const submitting = ref(false)

const onLogout = async () => {
  await withStandardizedError(() => authClient.signOut())
}

const onRequestOpenDoor = async () => {
  submitting.value = true
  await fetch('api/open-door', {
    method: 'POST',
    headers: { Authorization: `Bearer ${props.session.access_token}` },
  })
  submitting.value = false
}
</script>

<template>
  <form @submit.prevent="onLogout">
    <input type="submit" value="Abmelden" :disabled="loggingOut" />
  </form>

  <form @submit.prevent="onRequestOpenDoor">
    <input type="submit" value="Türe öffnen" :disabled="submitting" />
  </form>
</template>
