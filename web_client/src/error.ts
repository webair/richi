export const alwaysError = (e: unknown): Error => {
  if (!(e instanceof Error)) {
    return new Error(`unknown error: ${e}`)
  }
  return e
}
