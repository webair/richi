<script lang="ts" setup>
import type { Session } from '@supabase/supabase-js'
import { watch } from 'vue'

import { useAsyncState } from './shared/useAsyncState'
import { authClient, withStandardizedError } from './supabase'

const props = defineProps<{ session: Session }>()

const {
  state: lockRequestState,
  execute: requestOpenLock,
  reset: resetLockRequest,
} = useAsyncState(async (accessToken: string) => {
  await fetch('api/open-lock', {
    method: 'POST',
    headers: { Authorization: `Bearer ${accessToken}` },
  })
})

const onLogout = async () => {
  await withStandardizedError(() => authClient.signOut())
}

const onRequestOpenLock = async () => {
  requestOpenLock(props.session.access_token)
}

watch(lockRequestState, () => {
  if (lockRequestState.value.type === 'success-state') {
    setTimeout(() => {
      resetLockRequest()
    }, 5000)
  }
})
</script>

<template>
  <form @submit.prevent="onLogout">
    <input type="submit" value="Abmelden" />
  </form>

  <form
    v-if="lockRequestState.type === 'idle-state' || lockRequestState.type === 'error-state'"
    @submit.prevent="onRequestOpenLock"
  >
    <p v-if="lockRequestState.type === 'error-state'">
      Ups da ist etwas schief gelaufen: {{ lockRequestState.error.message }}
    </p>
    <input type="submit" value="Schloss öffnen" />
  </form>

  <p v-if="lockRequestState.type === 'processing-state'">Anfrage zum Öffnen wird gesendet...</p>

  <p v-if="lockRequestState.type === 'success-state'">Schloss wird geöffnet.</p>
</template>
