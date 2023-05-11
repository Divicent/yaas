const API_URL = 'http://127.0.0.1:8000/v1/';
const AUTH_TOKEN_KEY = 'auth_token'

export async function signIn(email: string, password: string): Promise<void> {
  const result = await fetch(`${API_URL}auth/sign-in`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ email, password })
  })

  if (result.ok){
    const body = await result.json() ;
    localStorage.setItem(AUTH_TOKEN_KEY, body.auth_token)
  } else {
    throw new Error(await result.text())
  }
}

export async function signUp(firstName: string, lastName: string, email: string, password: string) {
  const result = await fetch(`${API_URL}auth/sign-up`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ firstName, lastName, email, password })
  })

  if (!result.ok){
    throw new Error(await result.text())
  }
} 

export function isSignedIn(): boolean  {
  return localStorage.getItem(AUTH_TOKEN_KEY) !== null
}