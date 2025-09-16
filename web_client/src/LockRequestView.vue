<script lang="ts" setup>
import type { Session } from '@supabase/supabase-js'
import { watch } from 'vue'

import { useAsyncState } from './shared/useAsyncState'
import { webserviceRequest } from './shared/webservice-request'

const props = defineProps<{ session: Session }>()

const {
  state: lockRequestState,
  execute: requestOpenLock,
  reset: resetLockRequest,
} = useAsyncState(async (accessToken: string) =>
  webserviceRequest('api/open-lock', {
    method: 'POST',
    headers: { Authorization: `Bearer ${accessToken}` },
  }),
)

const onRequestOpenLock = async () => {
  requestOpenLock(props.session.access_token)
}

watch(lockRequestState, () => {
  if (lockRequestState.value.type === 'successState') {
    setTimeout(() => {
      resetLockRequest()
    }, 5000)
  }
})
</script>
<template>
  <form
    v-if="lockRequestState.type === 'idleState' || lockRequestState.type === 'errorState'"
    @submit.prevent="onRequestOpenLock"
  >
    <p v-if="lockRequestState.type === 'errorState'">
      Ups da ist etwas schief gelaufen: {{ lockRequestState.error.message }}
    </p>
    <input type="submit" value="Schloss öffnen" />
  </form>

  <p v-if="lockRequestState.type === 'processingState'">Anfrage zum Öffnen wird gesendet...</p>

  <p v-if="lockRequestState.type === 'successState'">{{ lockRequestState.data }}</p>
</template>
