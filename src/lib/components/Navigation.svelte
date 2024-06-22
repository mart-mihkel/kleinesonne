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

<div class="flex flex-row gap-8 p-2 dark:bg-black">
    <a
        class="transition-colors duration-300 hover:text-slate-500 dark:text-white"
        href="/"
        title="News">News</a
    >
    <Dropdown items={puppies}>Puppies</Dropdown>
    <Dropdown items={puppies}>Dogs</Dropdown>
    <Dropdown items={puppies}>Litters</Dropdown>
    <a
        class="transition-colors duration-300 hover:text-slate-500 dark:text-white"
        href="/"
        title="Contact page">Contact</a
    >
</div>
