<script lang="ts">
  import {Router, Route, Link} from 'svelte-navigator'
  import { invoke } from '@tauri-apps/api/tauri';
  import { message } from '@tauri-apps/api/dialog';
  import Home from './Home.svelte'
	import AddPw from './AddPw.svelte';
	import View from './View.svelte';
	import EditPw from './EditPw.svelte';
	import { masterPassword } from './utils/stores';

  // @ts-ignore
  const isTauri = typeof window !== "undefined" && window.__TAURI__;

  const EXPIRATION_TIME = 10 * 60 * 1000; // 10 minutes in milliseconds

  let timeoutId;

  function setMasterPassword(password) {
    masterPassword.set(password);
    clearTimeout(timeoutId);
    localStorage.setItem("masterPassword", password);
    localStorage.setItem("masterPassword_time", (new Date().getTime() + EXPIRATION_TIME).toString());
    timeoutId = setTimeout(() => {
      masterPassword.set('');
    }, EXPIRATION_TIME);
  }

  function checkMasterPassword() {
    const storedPassword = localStorage.getItem('masterPassword');
    if (storedPassword) {
      const storedTime = localStorage.getItem('masterPassword_time');
      const currentTime = new Date().getTime();
      if (currentTime - parseInt(storedTime) <= EXPIRATION_TIME) {
        invoke('validate_master_password', {password: storedPassword}).then((res) => {
          if (res) {
            masterPassword.set(storedPassword);
            timeoutId = setTimeout(() => {
              masterPassword.set('');
            }, EXPIRATION_TIME);
            return true;
          } else {
            localStorage.removeItem('masterPassword');
            localStorage.removeItem('masterPassword_time');
            return false;
          }
        });
      } else {
        localStorage.removeItem('masterPassword');
        localStorage.removeItem('masterPassword_time');
      }
    }
    return false;
  }

  checkMasterPassword();
</script>

<main>
  {#if !isTauri}
    <div class="alert alert-warning" role="alert">
      Diese App funktioniert nur innerhalb von der Tauri-App! Bitte starte die App mit <code>npm run tauri dev</code>! hier werden nur dummy Daten angezeigt!
    </div>
  {/if}
  {#if $masterPassword.length === 0 && isTauri}
  <div class="row">
    <h1>Masterpasswort</h1>
  </div>
  <div class="contents">
    <h3>Bitte Masterpasswort eingeben</h3>
    <form
      on:submit|preventDefault={(e) => {
        if (!isTauri) {
          alert("Diese Funktion ist nur in der Desktop App verfügbar!");
          return;
        }
        if (e.target[0].value.length === 0) {
          message("Bitte ein Passwort eingeben!");
          return;
        }
        invoke('validate_master_password', {password: e.target[0].value}).then((res) => {
          if (res) {
            setMasterPassword(e.target[0].value);
          } else {
            message("Falsches Passwort!");
          }
        });
      }}>
      <input
        type="password"
        placeholder="Masterpasswort"
      />
      <button type="submit">Bestätigen</button>
  </div>
  {:else}
  <Router>
    <Route path="/" component={Home} />
    <Route path="/addPw" component={AddPw} />
    <Route path="/viewPw" component={View} />
    <Route path="/editPw" component={EditPw} />
  </Router>
  {/if}
</main>