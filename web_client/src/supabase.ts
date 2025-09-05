import { AuthError, createClient } from '@supabase/supabase-js'

const SUPABASE_URL = 'https://kaiupgcdozjalfocsddp.supabase.co'
const SUPABASE_PUBLISHABLE_KEY = 'sb_publishable_IwXaHmHzHjwtr3WrsLATUA_gSy3ompQ'

export const authClient = createClient(SUPABASE_URL, SUPABASE_PUBLISHABLE_KEY).auth

export const withStandardizedError = async <T>(
  clientFunction: () => Promise<{ data: T; error: AuthError | null }>,
) => {
  const { data, error } = await clientFunction()
  if (error) {
    throw error
  }
  return data
}
