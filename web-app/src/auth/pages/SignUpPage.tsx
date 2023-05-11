import { Component, createSignal } from "solid-js";
import { signUp } from "../services/auth.service";
import toast from "solid-toast";
import { useNavigate } from "@solidjs/router";

const SignUpPage: Component = () => {

  const [firstName, setFirstName] = createSignal("")
  const [lastName, setLastName] = createSignal("")
  const [email, setEmail] = createSignal("")
  const [password, setPassword] = createSignal("")

  const naviaget = useNavigate()

  async function handleSignUp() {
    await toast.promise(
      signUp(firstName(), lastName(), email(), password()),
      {
        loading: "Signing up...",
        success: "Sign up successful!",
        error: (err) => err.message
      }
    ).then(() => {
      naviaget("/sign-in")
    })
  }

  return (
    <div>

      <div>
        <label>First Name</label>
        <input onInput={e => setFirstName(e.target.value)} value={firstName()} />
      </div>


      <div>
        <label>Last Name</label>
        <input onInput={e => setLastName(e.target.value)} value={lastName()} />
      </div>


      <div>
        <label>Email</label>
        <input type="email" onInput={e => setEmail(e.target.value)} value={email()} />
      </div>


      <div>
        <label>Password</label>
        <input type="password" onInput={e => setPassword(e.target.value)} value={password()} />
      </div>

      <button disabled={!firstName() || !lastName() || !email() || !password()} onClick={handleSignUp}>Sign Up</button>
    </div>
  )
}

export default SignUpPage