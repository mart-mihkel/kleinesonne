<script lang="ts">
    import type { PageData } from "./$types";
    import { Litter } from "$lib/components";

    export let data: PageData;

    // TODO: mobile list
    $: litter = data.litter;
</script>

<h2 class="p-4 text-center text-4xl">Litters</h2>
<div class="flex flex-col md:flex-row md:px-[5%] lg:px-[25%]">
    <div
        class="hidden flex-col border-t border-t-black md:flex md:w-1/4 dark:border-t-white"
    >
        {#each data.names as name}
            <a
                class="text-nowrap p-2"
                class:bg-black={litter?.name === name}
                class:text-white={litter?.name === name}
                class:dark:bg-white={litter?.name === name}
                class:dark:text-black={litter?.name === name}
                class:hover:bg-gray-300={litter?.name !== name}
                class:hover:dark:bg-gray-600={litter?.name !== name}
                href="/litters/{name}"
            >
                {name}
            </a>
        {/each}
    </div>
    {#if litter}
        {#key litter}
            <div class="md:w-3/4">
                <Litter {litter} />
            </div>
        {/key}
    {/if}
</div>
