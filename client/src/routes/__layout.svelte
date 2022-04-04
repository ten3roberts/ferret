<script>
  import "../app.css";
  import Header from "../lib/Header.svelte";
</script>

<Header />
<slot />

<style>
  :global(body) {
    /* background: linear-gradient(var(--primary-color), var(--secondary-color)); */
    background: linear-gradient(#2e3440, #5e81ac);
    background-size: cover;
  }
</style>
