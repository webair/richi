export const webserviceRequest = async (url: string, options?: RequestInit): Promise<string> => {
  const response = await fetch(url, options)
  if (!response.ok) {
    let errorBody: string
    try {
      errorBody = await response.text()
    } catch {
      errorBody = ''
    }

    throw new Error(`${errorBody} (${response.status})`)
  }
  return response.text()
}
