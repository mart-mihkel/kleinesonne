<script lang="ts">
    import "../app.css";
    import Header from "$lib/components/Header.svelte";
    import Navigation from "$lib/components/Navigation.svelte";
    import Footer from "$lib/components/Footer.svelte";
</script>

<Header />
<Navigation />
<slot />
<Footer />
