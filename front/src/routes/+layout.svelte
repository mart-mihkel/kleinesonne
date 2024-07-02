<script lang="ts">
    import "../app.css";
    import { Header, Footer } from "$lib/components";
</script>

<svelte:head>
    <title>Kleine Sonne</title>
    <meta
        name="description"
        content="Australian shepherd and Entlebucher mountain dog puppies available"
    />
</svelte:head>

<Header />
<main class="dark:bg-black dark:text-white">
    <slot />
</main>
<Footer />
