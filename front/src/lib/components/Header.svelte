<script lang="ts">
    import Dropdown from "./Dropdown.svelte";
    import { toggle, isDark } from "$lib/dark-mode";
    import { ShortBreed } from "$lib/types";
    import { onMount } from "svelte";

    const puppies = [
        {
            href: "/puppies/" + ShortBreed.AUSTRALIAN,
            text: "Australian shepherd puppies",
            title: "Australian shepherd puppies",
        },
        {
            href: "/puppies/" + ShortBreed.BERNESE,
            text: "Bernese mountain dog puppies",
            title: "Bernese mountain dog puppies",
        },
        {
            href: "/puppies/" + ShortBreed.ENTLEBUCHER,
            text: "Entlebucher mountain dog puppies",
            title: "Entlebucher mountain dog puppies",
        },
    ];

    const dogs = [
        {
            href: "/dogs/" + ShortBreed.AUSTRALIAN,
            text: "Australian shepherds",
            title: "Australian shepherds",
        },
        {
            href: "/dogs/" + ShortBreed.BERNESE,
            text: "Bernese mountain dogs",
            title: "Bernese mountain dogs",
        },
        {
            href: "/dogs/" + ShortBreed.ENTLEBUCHER,
            text: "Entlebucher mountain dogs",
            title: "Entlebucher mountain dogs",
        },
    ];

    let dark = false;
    onMount(() => (dark = isDark()));
</script>

<header
    class="flex w-full flex-row items-center gap-2 p-2 md:px-44 lg:px-64 dark:bg-black dark:text-white"
>
    <a
        class="text-2xl transition-colors duration-300 hover:text-slate-500"
        href="/"
        title="Kleine Sonne"
    >
        Kleine Sonne
    </a>
    <div class="flex-grow"></div>
    <button on:click={() => (dark = toggle())}>
        <svg
            xmlns="http://www.w3.org/2000/svg"
            height="24px"
            viewBox="0 -960 960 960"
            width="24px"
            class="fill-black transition-colors duration-300 hover:fill-slate-500 dark:fill-white"
        >
            {#if dark}
                <path
                    d="M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q14 0 27.5 1t26.5 3q-41 29-65.5 75.5T444-660q0 90 63 153t153 63q55 0 101-24.5t75-65.5q2 13 3 26.5t1 27.5q0 150-105 255T480-120Zm0-80q88 0 158-48.5T740-375q-20 5-40 8t-40 3q-123 0-209.5-86.5T364-660q0-20 3-40t8-40q-78 32-126.5 102T200-480q0 116 82 198t198 82Zm-10-270Z"
                />
            {:else}
                <path
                    d="M480-360q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35Zm0 80q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM200-440H40v-80h160v80Zm720 0H760v-80h160v80ZM440-760v-160h80v160h-80Zm0 720v-160h80v160h-80ZM256-650l-101-97 57-59 96 100-52 56Zm492 496-97-101 53-55 101 97-57 59Zm-98-550 97-101 59 57-100 96-56-52ZM154-212l101-97 55 53-97 101-59-57Zm326-268Z"
                />
            {/if}
        </svg>
    </button>
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
