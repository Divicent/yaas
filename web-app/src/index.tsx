/* @refresh reload */
import { render } from 'solid-js/web';

import './index.css';
import App from './App';
import { Outlet, Route, Router, Routes, useNavigate } from "@solidjs/router";
import SignIn from "./auth/pages/SignInPage"
import { Toaster } from "solid-toast"
import SignUp from './auth/pages/SignUpPage';
import { createEffect } from 'solid-js';
import { isSignedIn } from './auth/services/auth.service';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?',
  );
}

const RouteGuard = () => {
  const navigate = useNavigate()
  const isLoggedIn = isSignedIn()

  createEffect(() => {
    if (!isLoggedIn) {
      navigate("/sign-in")
    }
  })

  return (
    <Outlet />
  )
}

render(() => (
  <Router>
    <Toaster />
    <Routes>
      <Route path="/sign-in" component={SignIn} />
      <Route path="/sign-up" component={SignUp} />
      <Route path="/" component={RouteGuard}>
        <Route path="/" component={App} />
      </Route>
      <Route path="*" element={<div>Not Found</div>} />
    </Routes>
  </Router>
), root!);
