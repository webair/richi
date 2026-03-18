import { ref } from 'vue'

import { useAuthClient } from '@/shared/auth'
import { alwaysError } from '@/shared/error'

export interface EnterPhoneNumberState {
  type: 'enterPhoneNumberState'
  error?: string
  submitting: boolean
}

export interface EnterVerificationCodeState {
  type: 'verificationCodeState'
  error?: string
  submitting: boolean
}

export type LoginState = EnterPhoneNumberState | EnterVerificationCodeState

export const isEnterPhoneNumberState = (state: LoginState): state is EnterPhoneNumberState => {
  return state.type === 'enterPhoneNumberState'
}

export const isEnterVerificationCodeState = (
  state: LoginState
): state is EnterVerificationCodeState => {
  return state.type === 'verificationCodeState'
}

export const useLogin = () => {
  const authClient = useAuthClient()
  const state = ref<LoginState>({ type: 'enterPhoneNumberState', submitting: false })

  const submitPhoneNumber = async (phoneNumber: string) => {
    state.value = { type: 'enterPhoneNumberState', submitting: true }
    try {
      await authClient.loginWithPhoneNumber(phoneNumber)
      state.value = { type: 'verificationCodeState', submitting: false }
    } catch (e: unknown) {
      const error = alwaysError(e)
      state.value = {
        type: 'enterPhoneNumberState',
        error: error.message,
        submitting: false,
      }
    }
  }

  const submitVerificationCode = async (phoneNumber: string, verificationCode: string) => {
    state.value = { type: 'verificationCodeState', submitting: true }
    try {
      await authClient.verifyPhoneNumber(phoneNumber, verificationCode)
      state.value = { type: 'verificationCodeState', submitting: false }
    } catch (e) {
      const error = alwaysError(e)
      state.value = {
        type: 'verificationCodeState',
        error: error.message,
        submitting: false,
      }
    }
  }

  return {
    state,
    submitPhoneNumber,
    submitVerificationCode,
  }
}
