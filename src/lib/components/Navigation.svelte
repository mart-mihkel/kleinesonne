<script lang="ts">
    import Dropdown from "./Dropdown.svelte";

    const puppies = [
        {
            href: "/",
            text: "Bernese mountain dog puppies",
            title: "Bernese mountain dog puppies",
        },
        {
            href: "/",
            text: "Entlebucher cattle dog puppies",
            title: "Entlebucher cattle dog puppies",
        },
        {
            href: "/",
            text: "Australian shepherd puppies",
            title: "Australian shepherd puppies",
        },
    ];
</script>

<nav
    class="flex flex-row gap-8 border-b border-b-black md:px-44 lg:px-64 dark:bg-black dark:text-white"
>
    <a
        class="transition-colors duration-300 hover:text-slate-500"
        href="/"
        title="News"
    >
        News
    </a>
    <Dropdown items={puppies}>Puppies</Dropdown>
    <Dropdown items={puppies}>Dogs</Dropdown>
    <Dropdown items={puppies}>Litters</Dropdown>
    <a
        class="transition-colors duration-300 hover:text-slate-500"
        href="/contact"
        title="Contact page"
    >
        Contact
    </a>
</nav>
