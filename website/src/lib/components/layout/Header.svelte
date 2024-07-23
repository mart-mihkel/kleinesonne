<script lang="ts">
    import Facebook from "$lib/svg/Facebook.svelte";
    import Locale from "./Locale.svelte";
    import MobileNav from "./MobileNav.svelte";
    import Nav from "./Nav.svelte";
    import ThemeToggle from "./ThemeToggle.svelte";
</script>

<header
    class="flex w-full flex-row items-center gap-2 p-2 md:px-[5%] lg:px-[25%] dark:bg-black dark:text-white"
>
    <a
        class="text-2xl font-bold transition-colors duration-300 hover:text-gray-500"
        href="/"
    >
        Kleine Sonne
    </a>
    <span class="flex-grow"></span>
    <ThemeToggle />
    <Locale />
    <a
        href="https://www.facebook.com/p/Kleine-Sonne-100064871220699/"
        target="_blank"
    >
        <Facebook />
    </a>
</header>
<div class="border-b border-black dark:border-white dark:bg-black">
    <MobileNav />
    <Nav />
</div>
