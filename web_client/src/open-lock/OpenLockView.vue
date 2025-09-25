<script lang="ts" setup>
import type { Session } from '@supabase/supabase-js'
import { watch } from 'vue'

import {
  isErrorState,
  isIdleState,
  isProcessingState,
  isSuccessState,
  useAsyncState,
} from '@/shared/async-state'
import { webserviceRequest } from '@/shared/webservice-request'

const props = defineProps<{ session: Session }>()

const {
  state: lockRequestState,
  execute: requestOpenLock,
  reset: resetLockRequest,
} = useAsyncState(async (accessToken: string) =>
  webserviceRequest('api/open-lock', {
    method: 'POST',
    headers: { Authorization: `Bearer ${accessToken}` },
  })
)

const onRequestOpenLock = async () => {
  requestOpenLock(props.session.access_token)
}

watch(lockRequestState, () => {
  if (isSuccessState(lockRequestState.value)) {
    setTimeout(() => {
      resetLockRequest()
    }, 5000)
  }
})
</script>
<template>
  <form
    v-if="isIdleState(lockRequestState) || isErrorState(lockRequestState)"
    @submit.prevent="onRequestOpenLock"
  >
    <p v-if="isErrorState(lockRequestState)">
      Ups da ist etwas schief gelaufen: {{ lockRequestState.error.message }}
    </p>
    <input type="submit" value="Schloss öffnen" />
  </form>

  <p v-if="isProcessingState(lockRequestState)">Anfrage zum Öffnen wird gesendet...</p>

  <p v-if="isSuccessState(lockRequestState)">{{ lockRequestState.data }}</p>
</template>
