<script>
  import "../app.css";
  import Header from "../lib/Header.svelte";
</script>

<Header />
<slot />

<style>
  :global(body) {
    /* background: linear-gradient(var(--primary-color), var(--secondary-color)); */
    background-color: #5e81ac;
    background-repeat: no-repeat;
    background-attachment: fixed;
  }
</style>
