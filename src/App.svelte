<script lang="ts">
  // @ts-nocheck	workaround for TS complaining about the event targets
  import {Router, Route, Link} from 'svelte-navigator'
  import { invoke } from '@tauri-apps/api/tauri';
  import { message } from '@tauri-apps/api/dialog';
  import Home from './Home.svelte'
	import AddPw from './AddPw.svelte';
	import View from './View.svelte';
	import EditPw from './EditPw.svelte';
	import { masterPassword, theme, isSettingsOpen } from './utils/stores';
	import { updateTheme } from './utils/utillities';
	import SettingsDialog from './components/SettingsDialog.svelte';
	import ChangeMasterPw from './ChangeMasterPw.svelte';

  // @ts-ignore
  const isTauri = typeof window !== "undefined" && window.__TAURI__;

  const EXPIRATION_TIME = 10 * 60 * 1000; // 10 minutes in milliseconds

  let timeoutId: NodeJS.Timeout;

  function setMasterPassword(password: string) {
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
      const storedTime = localStorage.getItem('masterPassword_time') || '0';
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

  function detectColorScheme() {
    const themeStorage = localStorage.getItem('theme');
    if (themeStorage) {
      theme.set(themeStorage);
      updateTheme(themeStorage);
    } else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      theme.set('dark');
      updateTheme('dark');
    } else {
      theme.set('light');
      updateTheme('light');
    }
  }

  detectColorScheme();

  document.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 't') {
      theme.set($theme === "light" ? "dark" : "light");
      localStorage.setItem("theme", $theme);
      updateTheme($theme);
    }
    if (e.ctrlKey && e.key === 's' && $masterPassword.length > 0) {
      isSettingsOpen.set(true);
    }
  });
</script>

<main>
  <SettingsDialog isOpen={isSettingsOpen}>
    <button class="btn" on:click={() => {
      isSettingsOpen.set(false);
      location.href = '/cmpw';
    }}>
      Masterpasswort ändern
    </button>
    <button class="btn" on:click={() => {
      theme.set($theme === "light" ? "dark" : "light");
      localStorage.setItem("theme", $theme);
      updateTheme($theme);
    }}>
      Theme ändern
    </button>
  </SettingsDialog>
  {#if !isTauri}
    <div class="alert alert-warning" role="alert">
      Diese App funktioniert nur innerhalb von der Tauri-App! Bitte starte die App mit <code>npm run tauri dev</code>! Hier werden nur Dummy-Daten angezeigt!
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
        if (e.target === null || e.target[0] === null || e.target[1] === null) {
          message("Bitte ein Passwort eingeben!");
          return;
        }

        if (e.target[0].value.length === 0) {
          message("Bitte ein Passwort eingeben!");
          return;
        }
        const password = e.target[0].value;
        e.target[0].disabled = true;
        e.target[0].placeholder = "Login wird verifiziert...";
        e.target[0].value = "";
        e.target[1].disabled = true;
        console.log(e.target[0])
        console.log(e.target[1])
        invoke('validate_master_password', {password}).then((res) => {
          e.target[0].disabled = false;
          e.target[1].disabled = false;
          if (res) {
            setMasterPassword(password);
          } else {
            e.target[0].placeholder = "Masterpasswort";
            e.target[0].value = password;
            message("Falsches Passwort!");
          }
        });
      }}>
      <input
        type="password"
        placeholder="Masterpasswort"
      />
      <button class="btn " type="submit">Bestätigen</button>
    </form>
    <p class="tips">
      <span class="tip">Tipp: Du kannst <code>Strg + T</code> drücken, um das Theme zu wechseln.</span><br />
      <span class="tip">Tipp: Du kannst <code>Strg + S</code> drücken, um die Einstellungen zu öffnen. (Nur nach Eingabe des Masterpassworts)</span>
    </p>
  </div>
  {:else}
  <Router>
    <Route path="/" component={Home} />
    <Route path="/addPw" component={AddPw} />
    <Route path="/viewPw" component={View} />
    <Route path="/editPw" component={EditPw} />
    <Route path="/cmpw" component={ChangeMasterPw} />
  </Router>
  {/if}
</main>