<script lang="ts">
    import { Dropdown, ThemeToggle, Locale } from "$lib/components";
    import { Facebook } from "$lib/components/svg";
    import { Breed } from "$lib/types";
    import { format } from "svelte-i18n";

    $: puppies = [
        {
            href: "/puppies/" + Breed.AUSTRALIAN,
            text: $format("puppies.aus"),
        },
        {
            href: "/puppies/" + Breed.ENTLEBUCH,
            text: $format("puppies.ent"),
        },
        {
            href: "/puppies/" + Breed.BERNESE,
            text: $format("puppies.ber"),
        },
    ];

    $: dogs = [
        {
            href: "/dogs/" + Breed.AUSTRALIAN,
            text: $format("breed.aus.many"),
        },
        {
            href: "/dogs/" + Breed.ENTLEBUCH,
            text: $format("breed.ent.many"),
        },
        {
            href: "/dogs/" + Breed.BERNESE,
            text: $format("breed.ber.many"),
        },
    ];
</script>

<header
    class="flex w-full flex-row items-center gap-2 p-2 md:px-[5%] lg:px-[25%] dark:bg-black dark:text-white"
>
    <a
        class="hover:text-g6ray-500 text-2xl transition-colors duration-300"
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
<nav
    class="flex flex-row gap-8 border-b border-black md:px-[5%] lg:px-[25%] dark:border-white dark:bg-black dark:text-white"
>
    <Dropdown href="/puppies" items={puppies}>{$format("nav.puppies")}</Dropdown
    >
    <Dropdown href="/dogs" items={dogs}>{$format("nav.dogs")}</Dropdown>
    <a
        class="transition-colors duration-300 hover:text-gray-500"
        href="/litters"
    >
        {$format("nav.litters")}
    </a>
    <a class="transition-colors duration-300 hover:text-gray-500" href="/news">
        {$format("nav.news")}
    </a>
    <a
        class="transition-colors duration-300 hover:text-gray-500"
        href="/contact"
    >
        {$format("nav.contact")}
    </a>
</nav>
