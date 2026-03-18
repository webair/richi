import { type AuthChangeEvent, AuthError, createClient, type Session } from '@supabase/supabase-js'

const SUPABASE_URL = 'https://kaiupgcdozjalfocsddp.supabase.co'
const SUPABASE_PUBLISHABLE_KEY = 'sb_publishable_IwXaHmHzHjwtr3WrsLATUA_gSy3ompQ'

const supabaseAuthClient = createClient(SUPABASE_URL, SUPABASE_PUBLISHABLE_KEY).auth

export const useAuthClient = () => {
  const loginWithPhoneNumber = async (phoneNumber: string) =>
    withStandardizedError(() =>
      supabaseAuthClient.signInWithOtp({
        phone: phoneNumber,
        options: { channel: 'sms' },
      })
    )

  const verifyPhoneNumber = async (phoneNumber: string, verificationCode: string) => {
    return withStandardizedError(() =>
      supabaseAuthClient.verifyOtp({
        token: verificationCode,
        phone: phoneNumber,
        type: 'sms',
      })
    )
  }
  const logout = async () => {
    return withStandardizedError(() => supabaseAuthClient.signOut())
  }

  const getSession = async () => {
    const { session } = await withStandardizedError(() => supabaseAuthClient.getSession())
    return session
  }

  const onAuthStateChange: typeof supabaseAuthClient.onAuthStateChange = (
    callback: (event: AuthChangeEvent, session: Session | null) => void
  ) => supabaseAuthClient.onAuthStateChange(callback)

  return {
    loginWithPhoneNumber,
    verifyPhoneNumber,
    getSession,
    logout,
    onAuthStateChange,
  }
}

const withStandardizedError = async <T extends { data?: unknown; error: AuthError | null }>(
  clientFunction: () => Promise<T>
): Promise<T extends { data: infer R } ? R : void> => {
  const returnValue = await clientFunction()
  if (returnValue.error) {
    throw returnValue.error
  }
  return returnValue.data as T extends { data: infer R } ? R : void
}
