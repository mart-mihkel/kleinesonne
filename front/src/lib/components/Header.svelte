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

    const dogs = [
        {
            href: "/dogs/bernese/",
            text: "Bernese mountain dogs",
            title: "Bernese mountain dogs",
        },
        {
            href: "/dogs/entlebusher/",
            text: "Entlebucher cattle dogs",
            title: "Entlebucher cattle dogs",
        },
        {
            href: "/dogs/australian/",
            text: "Australian shepherds",
            title: "Australian shepherds",
        },
    ];
</script>

<header
    class="flex w-full flex-row items-center gap-8 p-2 md:px-44 lg:px-64 dark:bg-black"
>
    <a
        class="text-2xl transition-colors duration-300 hover:text-slate-500 dark:text-white"
        href="/"
        title="Kleine Sonne"
    >
        Kleine Sonne
    </a>
    <div class="flex-grow"></div>
    <a
        href="https://www.facebook.com/p/Kleine-Sonne-100064871220699/"
        target="_blank"
        title="Facbook"
    >
        <svg
            class="fill-black transition-colors duration-300 hover:fill-slate-500 dark:fill-white"
            xmlns="http://www.w3.org/2000/svg"
            x="0px"
            y="0px"
            width="24px"
            height="24px"
            viewBox="0 0 24 24"
        >
            <path
                d="M 5 3 C 3.897 3 3 3.897 3 5 L 3 19 C 3 20.103 3.897 21 5 21 L 11.621094 21 L 14.414062 21 L 19 21 C 20.103 21 21 20.103 21 19 L 21 5 C 21 3.897 20.103 3 19 3 L 5 3 z M 5 5 L 19 5 L 19.001953 19 L 14.414062 19 L 14.414062 15.035156 L 16.779297 15.035156 L 17.130859 12.310547 L 14.429688 12.310547 L 14.429688 10.574219 C 14.429687 9.7862188 14.649297 9.2539062 15.779297 9.2539062 L 17.207031 9.2539062 L 17.207031 6.8222656 C 16.512031 6.7512656 15.814234 6.71675 15.115234 6.71875 C 13.041234 6.71875 11.621094 7.9845938 11.621094 10.308594 L 11.621094 12.314453 L 9.2773438 12.314453 L 9.2773438 15.039062 L 11.621094 15.039062 L 11.621094 19 L 5 19 L 5 5 z"
            />
        </svg>
    </a>
</header>
<nav
    class="flex flex-row gap-8 border-b border-b-black md:px-44 lg:px-64 dark:border-b-white dark:bg-black dark:text-white"
>
    <a
        class="transition-colors duration-300 hover:text-slate-500"
        href="/"
        title="News"
    >
        News
    </a>
    <Dropdown items={puppies}>Puppies</Dropdown>
    <Dropdown items={dogs}>Dogs</Dropdown>
    <Dropdown items={puppies}>Litters</Dropdown>
    <a
        class="transition-colors duration-300 hover:text-slate-500"
        href="/contact"
        title="Contact page"
    >
        Contact
    </a>
</nav>
