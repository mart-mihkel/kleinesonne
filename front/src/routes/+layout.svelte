<script lang="ts">
    import "../app.css";
    import { Header, Footer } from "$lib/components";
</script>

<svelte:head>
    <title>Kleine Sonne</title>
    <meta
        name="description"
        content="Australian Shepherd and Entlebuch Cattle Dog puppies available"
    />
</svelte:head>

<div class="flex h-screen flex-col dark:bg-black dark:text-white">
    <Header />
    <main class="mb-auto dark:bg-black">
        <slot />
    </main>
    <Footer />
</div>
