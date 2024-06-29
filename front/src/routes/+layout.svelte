<script lang="ts">
    import "../app.css";
    import Header from "$lib/components/Header.svelte";
    import Footer from "$lib/components/Footer.svelte";
</script>

<svelte:head>
    <title>Kleine Sonne</title>
    <meta
        name="description"
        content="Australian shepherd and Entlebucher mountain dog puppies available"
    />
</svelte:head>

<Header />
<main>
    <slot />
</main>
<Footer />
