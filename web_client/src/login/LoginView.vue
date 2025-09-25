<script lang="ts" setup>
import { ref } from 'vue'

import { isEnterPhoneNumberState, isEnterVerificationCodeState, useLogin } from './login'
import PhoneNumberForm from './PhoneNumberForm.vue'
import VerificationCodeForm from './VerificationCodeForm.vue'

const { state, submitPhoneNumber, submitVerificationCode } = useLogin()
const phoneNumberInput = ref('')
const verificationCodeInput = ref('')

const onSubmitPhoneNumber = () => {
  submitPhoneNumber(phoneNumberInput.value)
}

const onSubmitVerificationCode = () => {
  submitVerificationCode(phoneNumberInput.value, verificationCodeInput.value)
}
</script>

<template>
  <PhoneNumberForm
    v-if="isEnterPhoneNumberState(state)"
    v-model="phoneNumberInput"
    :login-state="state"
    @submit.prevent="onSubmitPhoneNumber"
  />
  <VerificationCodeForm
    v-if="isEnterVerificationCodeState(state)"
    v-model="verificationCodeInput"
    :login-state="state"
    @submit.prevent="onSubmitVerificationCode"
  />
</template>
