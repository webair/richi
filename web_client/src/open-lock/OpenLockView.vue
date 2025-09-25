<script lang="ts" setup>
import { NAlert, NButton } from 'naive-ui'
import { watch } from 'vue'

import {
  isErrorState,
  isIdleState,
  isProcessingState,
  isSuccessState,
  useAsyncState,
} from '@/shared/async-state'
import type { Session } from '@/shared/auth'
import UiForm from '@/shared/ui/UiForm.vue'
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
  <UiForm
    v-if="isIdleState(lockRequestState) || isErrorState(lockRequestState)"
    @submit.prevent="onRequestOpenLock"
  >
    <NAlert v-if="isErrorState(lockRequestState)" title="Error" type="error">
      {{ lockRequestState.error.message }}
    </NAlert>
    <NButton type="primary" attr-type="submit" size="large"> Schloss öffnen </NButton>
  </UiForm>

  <p v-if="isProcessingState(lockRequestState)">Anfrage zum Öffnen wird gesendet...</p>

  <p v-if="isSuccessState(lockRequestState)">{{ lockRequestState.data }}</p>
</template>
