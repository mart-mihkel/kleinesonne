<script lang="ts">
    import Footer from "$lib/components/layout/Footer.svelte";
    import Header from "$lib/components/layout/Header.svelte";
    import Nav from "$lib/components/layout/Nav.svelte";
    import "../app.css";
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
