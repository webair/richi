<script lang="ts" setup>
import { NButton, NInput } from 'naive-ui'
import { computed, defineModel, defineProps } from 'vue'

import UiForm from '@/shared/ui/UiForm.vue'

import { type EnterPhoneNumberState } from './login'

const { loginState } = defineProps<{
  loginState: EnterPhoneNumberState
}>()

const phoneNumberInput = defineModel<string>({ default: '' })

const canSubmitPhoneNumber = computed(() => {
  return phoneNumberInput.value.length === 12 && !loginState.submitting
})
</script>

<template>
  <UiForm>
    <p>Bitte gebe deine Telefonnummer ein</p>
    <p v-if="loginState.error" class="error">{{ loginState.error }}</p>
    <NInput
      v-model:value="phoneNumberInput"
      :autofocus="true"
      type="text"
      attr-type="tel"
      placeholder="+41791236712"
      size="large"
      :readonly="loginState.submitting"
    />
    <NButton
      attr-type="submit"
      :disabled="!canSubmitPhoneNumber"
      :loading="loginState.submitting"
      size="large"
    >
      Anmelden
    </NButton>
  </UiForm>
</template>
