<script lang="ts">
  import { Router, Route } from 'svelte-routing';
  import Search from './routes/search/+page.svelte';
  import Timeline from './routes/timeline/+page.svelte';
  import Home from './routes/+page.svelte';
</script>

<main class="container">
  <Router>
    <Route path="" component={Home} />
    <Route path="search" component={Search} />
    <Route path="timeline" component={Timeline} />
  </Router>
</main>

<style>
</style>
