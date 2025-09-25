<script lang="ts" setup>
import { computed, defineModel, defineProps } from 'vue'

import UiForm from '@/shared/ui/UiForm.vue'

import { type EnterVerificationCodeState } from './login'

const { loginState } = defineProps<{
  loginState: EnterVerificationCodeState
}>()

const verificationCodeInput = defineModel<string>({ default: '' })

const canSubmitVerificationCode = computed(() => {
  return verificationCodeInput.value.length === 6 && !loginState.submitting
})
</script>

<template>
  <UiForm>
    <p>Bitte gib den 6-stelligen Code aus der SMS ein</p>
    <p v-if="loginState.error" class="error">{{ loginState.error }}</p>
    <input
      v-model="verificationCodeInput"
      type="tel"
      placeholder="123456"
      :readonly="loginState.submitting"
    />
    <input type="submit" value="Telefonummer verifizieren" :disabled="!canSubmitVerificationCode" />
  </UiForm>
</template>
