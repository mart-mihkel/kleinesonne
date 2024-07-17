<script lang="ts">
    import "../app.css";
    import { Header, Nav, Footer } from "$lib/components";
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
    <Nav />
    <main
        class="mb-auto bg-gradient-to-b from-slate-200 to-white dark:from-gray-900 dark:to-black"
    >
        <slot />
    </main>
    <Footer />
</div>
