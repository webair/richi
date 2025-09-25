<script lang="ts" setup>
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
    <input
      v-model="phoneNumberInput"
      type="tel"
      placeholder="+41791236712"
      :readonly="loginState.submitting"
    />
    <input type="submit" value="Anmelden" :disabled="!canSubmitPhoneNumber" />
  </UiForm>
</template>
