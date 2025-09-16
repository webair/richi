import { ref } from 'vue'

interface IdleState {
  type: 'idle-state'
}

interface ProcessingState {
  type: 'processing-state'
}

interface SuccessState<T> {
  type: 'success-state'
  data: T
}

interface ErrorState {
  type: 'error-state'
  error: Error
}

type AsyncState<T> = IdleState | ProcessingState | SuccessState<T> | ErrorState

export const useAsyncState = <T, Args extends unknown[] = []>(
  asyncFunction: (...args: Args) => Promise<T>,
) => {
  const state = ref<AsyncState<T>>({ type: 'idle-state' })

  const execute = async (...args: Args) => {
    state.value = { type: 'processing-state' }
    try {
      const data = await asyncFunction(...args)
      state.value = { type: 'success-state', data }
    } catch (error) {
      if (error instanceof Error) {
        state.value = { type: 'error-state', error }
      } else {
        state.value = { type: 'error-state', error: new Error(`${error}`) }
      }
    }
  }

  const reset = () => {
    if (state.value.type !== 'processing-state') {
      state.value = { type: 'idle-state' }
    }
  }

  return { state, execute, reset }
}
