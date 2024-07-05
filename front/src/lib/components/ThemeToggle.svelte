<script lang="ts">
    import { Sun, Moon } from "$lib/components/svg";
    import { onMount } from "svelte";

    let dark = false;

    function toggle() {
        dark = !dark;
        localStorage.setItem("theme", dark ? "dark" : "light");
        document.documentElement.classList.toggle("dark");
    }

    function listener(e: MediaQueryListEvent) {
        if (dark === e.matches) {
            return;
        }

        toggle();
    }

    onMount(() => {
        dark = localStorage.getItem("theme") === "dark";
        const query = window.matchMedia("(prefers-color-scheme: dark)");
        query.addEventListener("change", listener);
        return () => query.removeEventListener("change", listener);
    });
</script>

<button name="toggle" on:click={toggle}>
    {#if dark}
        <Moon />
    {:else}
        <Sun />
    {/if}
</button>
