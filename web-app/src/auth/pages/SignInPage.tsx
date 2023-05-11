import { Component, createSignal } from "solid-js"
import { signIn } from "./auth.service"

const SignInPage: Component = () => {
  const [email, setEmail] = createSignal("")
  const [password, setPassword]  = createSignal("")

  async function handleSignIn() {
    try {
      const result = await signIn(email(), password())
      console.log("RESULT", result)
    } catch (e){
      // TODO show error message
      console.log("ERRRO LOGIN", e)
    }
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