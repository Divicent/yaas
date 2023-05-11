const API_URL = 'http://127.0.0.1:8000/v1/';

export async function signIn(email: string, password: string): Promise<string> {
  const result = await fetch(`${API_URL}auth/sign-in`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ email, password })
  })

  if (result.ok){
    return result.json();
  } else {
    throw new Error(await result.text())
  }
}