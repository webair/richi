<script lang="ts" setup>
import type { Session } from '@supabase/supabase-js'

import { useAsyncState } from './shared/useAsyncState'
import { authClient, withStandardizedError } from './supabase'

const props = defineProps<{ session: Session }>()

const { state: requestOpenLockState, execute: requestOpenLock } = useAsyncState(
  async (accessToken: string) => {
    await fetch('api/open-lock', {
      method: 'POST',
      headers: { Authorization: `Bearer ${accessToken}` },
    })
  },
)

const onLogout = async () => {
  await withStandardizedError(() => authClient.signOut())
}

const onRequestOpenLock = async () => {
  requestOpenLock(props.session.access_token)
}
</script>

<template>
  <form @submit.prevent="onLogout">
    <input type="submit" value="Abmelden" />
  </form>

  <form v-if="requestOpenLockState.type === 'idle-state'" @submit.prevent="onRequestOpenLock">
    <input type="submit" value="Schloss öffnen" />
  </form>

  <p v-if="requestOpenLockState.type === 'processing-state'">Anfrage zum Öffnen wird gesendet...</p>

  <p v-if="requestOpenLockState.type === 'success-state'">Schloss wird geöffnet.</p>

  <p v-if="requestOpenLockState.type === 'error-state'">
    Ups da ist etwas schief gelaufen: {{ requestOpenLockState.error.message }}
  </p>
</template>
