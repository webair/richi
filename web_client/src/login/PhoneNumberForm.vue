<script lang="ts" setup>
import type { InputInst } from 'naive-ui'
import { NButton, NInput } from 'naive-ui'
import { computed, defineModel, defineProps, onMounted, ref } from 'vue'

import UiForm from '@/shared/ui/UiForm.vue'

import { type EnterPhoneNumberState } from './login'

const { loginState } = defineProps<{
  loginState: EnterPhoneNumberState
}>()

const inputValue = defineModel<string>({ default: '' })
const inputRef = ref<InputInst>()

const canSubmit = computed(() => inputValue.value.length === 12 && !loginState.submitting)

onMounted(() => {
  inputRef.value?.focus()
})
</script>

<template>
  <UiForm>
    <p>Bitte gebe deine Telefonnummer ein</p>
    <p v-if="loginState.error" class="error">{{ loginState.error }}</p>
    <NInput
      ref="inputRef"
      v-model:value="inputValue"
      :autofocus="true"
      type="text"
      attr-type="tel"
      placeholder="+41791236712"
      size="large"
      :readonly="loginState.submitting"
    />
    <NButton
      attr-type="submit"
      type="primary"
      :disabled="!canSubmit"
      :loading="loginState.submitting"
      size="large"
    >
      Anmelden
    </NButton>
  </UiForm>
</template>
