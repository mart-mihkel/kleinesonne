<script lang="ts">
    import { ThemeToggle, Locale } from "$lib/components";
    import { Facebook } from "$lib/svg";
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
