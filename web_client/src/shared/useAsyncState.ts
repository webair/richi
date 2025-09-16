import { ref } from 'vue'

interface IdleState {
  type: 'idleState'
}

interface ProcessingState {
  type: 'processingState'
}

interface SuccessState<T> {
  type: 'successState'
  data: T
}

interface ErrorState {
  type: 'errorState'
  error: Error
}

type AsyncState<T> = IdleState | ProcessingState | SuccessState<T> | ErrorState

export const useAsyncState = <T, Args extends unknown[] = []>(
  asyncFunction: (...args: Args) => Promise<T>
) => {
  const state = ref<AsyncState<T>>({ type: 'idleState' })

  const execute = async (...args: Args) => {
    state.value = { type: 'processingState' }
    try {
      const data = await asyncFunction(...args)
      state.value = { type: 'successState', data }
    } catch (error) {
      if (error instanceof Error) {
        state.value = { type: 'errorState', error }
      } else {
        state.value = { type: 'errorState', error: new Error(`${error}`) }
      }
    }
  }

  const reset = () => {
    if (state.value.type !== 'processingState') {
      state.value = { type: 'idleState' }
    }
  }

  return { state, execute, reset }
}
