<script lang="ts">
  import {Router, Route, Link} from 'svelte-navigator'
  import Home from './Home.svelte'
	import AddPw from './AddPw.svelte';
</script>

<main>
  <Router>
    <Route path="/" component={Home} />
    <Route path="/addPw" component={AddPw} />
  </Router>
</main>