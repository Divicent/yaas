import { Component, createSignal } from "solid-js"
import { signIn } from "../services/auth.service"
import toast from "solid-toast"

const SignInPage: Component = () => {
  const [email, setEmail] = createSignal("")
  const [password, setPassword] = createSignal("")

  async function handleSignIn() {
    await toast.promise(
      signIn(email(), password()), {
      loading: "Signing in...",
      success: null,
      error: (err) => err.message
    })
  }

  return (
    <div>
      <h3>Sign In</h3>

      <label>Email</label>
      <input onInput={e => setEmail(e.target.value)} value={email()} />

      <label>Password</label>
      <input onInput={e => setPassword(e.target.value)} value={password()} />

      <button disabled={!email() || !password()} onClick={handleSignIn}>Login</button>
    </div>
  )
}

export default SignInPage