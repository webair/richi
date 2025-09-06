<script lang="ts" setup>
import { computed, ref } from 'vue'

import { alwaysError } from './error'
import { authClient, withStandardizedError } from './supabase'

interface EnterPhoneNumberState {
  type: 'enterPhoneNumberState'
  error?: string
}

interface VerificationCodeState {
  type: 'verificationCodeState'
  error?: string
}

type LoginState = EnterPhoneNumberState | VerificationCodeState

const inputPhone = ref('')
const inputVerificationCode = ref('')
const state = ref<LoginState>({ type: 'enterPhoneNumberState' })
const submitting = ref(false)

const onSubmitPhoneNumber = async () => {
  submitting.value = true
  try {
    await withStandardizedError(() =>
      authClient.signInWithOtp({
        phone: inputPhone.value,
        options: { channel: 'sms' },
      }),
    )
    state.value = { type: 'verificationCodeState' }
  } catch (e) {
    const error = alwaysError(e)
    state.value = {
      type: 'enterPhoneNumberState',
      error: error.message,
    }
  } finally {
    submitting.value = false
  }
}

const onSubmitVerificationCode = async () => {
  submitting.value = true
  try {
    await withStandardizedError(() =>
      authClient.verifyOtp({
        token: inputVerificationCode.value,
        phone: inputPhone.value,
        type: 'sms',
      }),
    )
  } catch (e) {
    const error = alwaysError(e)
    state.value = {
      type: 'verificationCodeState',
      error: error.message,
    }
  } finally {
    submitting.value = false
  }
}

const canSubmitPhoneNumber = computed(() => {
  return inputPhone.value.length === 12 && !submitting.value
})

const canSubmitVerificationCode = computed(() => {
  return inputVerificationCode.value.length === 6 && !submitting.value
})
</script>

<template>
  <form
    v-if="state.type === 'enterPhoneNumberState'"
    class="login-view"
    @submit.prevent="onSubmitPhoneNumber"
  >
    <p>Bitte gebe deine Telefonnummer ein</p>
    <p v-if="state.error" class="error">{{ state.error }}</p>
    <input v-model="inputPhone" type="tel" placeholder="+41791236712" :readonly="submitting" />
    <input type="submit" value="Anmelden" :disabled="!canSubmitPhoneNumber" />
  </form>
  <form
    v-if="state.type === 'verificationCodeState'"
    class="login-view"
    @submit.prevent="onSubmitVerificationCode"
  >
    <p>Bitte gib den 6-stelligen Code aus der SMS ein</p>
    <p v-if="state.error" class="error">{{ state.error }}</p>
    <input v-model="inputVerificationCode" type="tel" placeholder="123456" />
    <input type="submit" value="Telefonummer verifizieren" :disabled="!canSubmitVerificationCode" />
  </form>
</template>

<style>
.login-view {
  display: flex;
  align-items: flex-start;
  flex-direction: column;
  gap: 0.5rem;

  .error {
    color: red;
  }
}
</style>
