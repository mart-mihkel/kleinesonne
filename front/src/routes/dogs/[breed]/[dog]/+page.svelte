<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;

    const { name } = data.dog;
</script>

<div class="flex flex-col md:px-44 lg:px-64 dark:bg-black dark:text-white">
    <h2 class="p-4 text-center text-4xl">{name}</h2>
    <div>statsid</div>
    <div>medalid</div>
    <div>gallery</div>
    <div>sugupuu</div>
</div>
