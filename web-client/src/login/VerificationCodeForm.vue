<script lang="ts" setup>
import type { InputInst } from 'naive-ui'
import { NButton, NInput } from 'naive-ui'
import { computed, defineModel, defineProps, onMounted, ref } from 'vue'

import UiForm from '@/shared/ui/UiForm.vue'

import { type EnterVerificationCodeState } from './login'

const { loginState } = defineProps<{
  loginState: EnterVerificationCodeState
}>()

const inputValue = defineModel<string>({ default: '' })
const inputRef = ref<InputInst>()

const canSubmit = computed(() => inputValue.value.length === 6 && !loginState.submitting)

onMounted(() => {
  inputRef.value?.focus()
})
</script>

<template>
  <UiForm>
    <p>Bitte gib den 6-stelligen Code aus der SMS ein</p>
    <p v-if="loginState.error" class="error">{{ loginState.error }}</p>
    <NInput
      ref="inputRef"
      v-model:value="inputValue"
      :autosize="true"
      type="text"
      attr-type="number"
      placeholder="123456"
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
      Telefonummer verifizieren
    </NButton>
  </UiForm>
</template>
