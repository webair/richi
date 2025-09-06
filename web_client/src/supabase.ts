import { AuthError, createClient } from '@supabase/supabase-js'

const SUPABASE_URL = 'https://kaiupgcdozjalfocsddp.supabase.co'
const SUPABASE_PUBLISHABLE_KEY = 'sb_publishable_IwXaHmHzHjwtr3WrsLATUA_gSy3ompQ'

export const authClient = createClient(SUPABASE_URL, SUPABASE_PUBLISHABLE_KEY).auth

export const withStandardizedError = async <T extends { data?: unknown; error: AuthError | null }>(
  clientFunction: () => Promise<T>,
): Promise<T extends { data: infer R } ? R : void> => {
  const returnValue = await clientFunction()
  if (returnValue.error) {
    throw returnValue.error
  }
  return returnValue.data as T extends { data: infer R } ? R : void
}
