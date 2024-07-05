<script lang="ts">
    import { Dropdown, ThemeToggle } from "$lib/components";
    import { Facebook } from "$lib/components/svg";
    import { Breed } from "$lib/types";

    const puppies = [
        {
            href: "/puppies/" + Breed.AUSTRALIAN,
            text: "Australian Shepherd puppies",
            title: "Australian Shepherd puppies",
        },
        {
            href: "/puppies/" + Breed.ENTLEBUCH,
            text: "Entlebuch Cattle Dog puppies",
            title: "Entlebuch Cattle Dog puppies",
        },
        {
            href: "/puppies/" + Breed.BERNESE,
            text: "Bernese Mountain Dog puppies",
            title: "Bernese Mountain Dog puppies",
        },
    ];

    const dogs = [
        {
            href: "/dogs/" + Breed.AUSTRALIAN,
            text: "Australian Shepherds",
            title: "Australian Shepherds",
        },
        {
            href: "/dogs/" + Breed.ENTLEBUCH,
            text: "Entlebuch Cattle Dogs",
            title: "Entlebuch Cattle Dogs",
        },
        {
            href: "/dogs/" + Breed.BERNESE,
            text: "Bernese Mountain Dogs",
            title: "Bernese Mountain Dogs",
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
    <Dropdown href="/puppies" items={puppies}>Puppies</Dropdown>
    <Dropdown href="/dogs" items={dogs}>Dogs</Dropdown>
    <a
        class="transition-colors duration-300 hover:text-gray-500"
        href="/litters"
    >
        Litters
    </a>
    <a class="transition-colors duration-300 hover:text-gray-500" href="/news">
        News
    </a>
    <a
        class="transition-colors duration-300 hover:text-gray-500"
        href="/contact"
    >
        Contact
    </a>
</nav>
