<script lang="ts">
  import {Router, Route, Link} from 'svelte-navigator'
  import Home from './Home.svelte'
	import AddPw from './AddPw.svelte';
	import View from './View.svelte';
	import EditPw from './EditPw.svelte';
</script>

<main>
  <Router>
    <Route path="/" component={Home} />
    <Route path="/addPw" component={AddPw} />
    <Route path="/viewPw" component={View} />
    <Route path="/editPw" component={EditPw} />
  </Router>
</main>