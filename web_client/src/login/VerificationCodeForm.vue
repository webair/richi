<script lang="ts" setup>
import { NButton, NInput } from 'naive-ui'
import { computed, defineModel, defineProps } from 'vue'

import UiForm from '@/shared/ui/UiForm.vue'

import { type EnterVerificationCodeState } from './login'

const { loginState } = defineProps<{
  loginState: EnterVerificationCodeState
}>()

const verificationCodeInput = defineModel<string>({ default: '' })

const canSubmitVerificationCode = computed(
  () => verificationCodeInput.value.length === 6 && !loginState.submitting
)
</script>

<template>
  <UiForm>
    <p>Bitte gib den 6-stelligen Code aus der SMS ein</p>
    <p v-if="loginState.error" class="error">{{ loginState.error }}</p>
    <NInput
      v-model:value="verificationCodeInput"
      type="text"
      attr-type="tel"
      placeholder="123456"
      size="large"
      :readonly="loginState.submitting"
    />
    <NButton
      attr-type="submit"
      type="primary"
      :disabled="!canSubmitVerificationCode"
      :loading="loginState.submitting"
      size="large"
    >
      Telefonummer verifizieren
    </NButton>
  </UiForm>
</template>
