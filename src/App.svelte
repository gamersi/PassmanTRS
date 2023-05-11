<script lang="ts">
  import {Router, Route, Link} from 'svelte-navigator'
  import Home from './Home.svelte'
	import AddPw from './AddPw.svelte';
	import View from './View.svelte';
	import EditPw from './EditPw.svelte';

  // @ts-ignore
  const isTauri = typeof window !== "undefined" && window.__TAURI__;
</script>

<main>
  {#if !isTauri}
    <div class="alert alert-warning" role="alert">
      Diese App funktioniert nur innerhalb von der Tauri-App! Bitte starte die App mit <code>npm run tauri dev</code>! hier werden nur dummy Daten angezeigt!
    </div>
  {/if}
  <Router>
    <Route path="/" component={Home} />
    <Route path="/addPw" component={AddPw} />
    <Route path="/viewPw" component={View} />
    <Route path="/editPw" component={EditPw} />
  </Router>
</main>