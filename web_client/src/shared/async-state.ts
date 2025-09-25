import { ref } from 'vue'

interface IdleState {
  type: 'idle'
}

interface ProcessingState {
  type: 'processing'
}

interface SuccessState<T> {
  type: 'success'
  data: T
}

interface ErrorState {
  type: 'error'
  error: Error
}

type AsyncState<T> = IdleState | ProcessingState | SuccessState<T> | ErrorState

export const isIdleState = <T>(asyncState: AsyncState<T>): asyncState is IdleState => {
  return asyncState.type === 'idle'
}

export const isProcessingState = <T>(asyncState: AsyncState<T>): asyncState is ProcessingState => {
  return asyncState.type === 'processing'
}

export const isSuccessState = <T>(asyncState: AsyncState<T>): asyncState is SuccessState<T> => {
  return asyncState.type === 'success'
}

export const isErrorState = (asyncState: AsyncState<unknown>): asyncState is ErrorState => {
  return asyncState.type === 'error'
}

export const useAsyncState = <T, Args extends unknown[] = []>(
  asyncFunction: (...args: Args) => Promise<T>
) => {
  const state = ref<AsyncState<T>>({ type: 'idle' })

  const execute = async (...args: Args) => {
    state.value = { type: 'processing' }
    try {
      const data = await asyncFunction(...args)
      state.value = { type: 'success', data }
    } catch (error) {
      if (error instanceof Error) {
        state.value = { type: 'error', error }
      } else {
        state.value = { type: 'error', error: new Error(`${error}`) }
      }
    }
  }

  const reset = () => {
    if (state.value.type !== 'processing') {
      state.value = { type: 'idle' }
    }
  }

  return { state, execute, reset }
}
