<script lang="ts">
    import LitterDisplay from "$lib/components/litter/LitterDisplay.svelte";
    import LitterList from "$lib/components/litter/LitterList.svelte";
    import Loading from "$lib/components/notice/Loading.svelte";
    import Empty from "$lib/components/notice/Empty.svelte";
    import Error from "$lib/components/notice/Error.svelte";
    import type { PageServerData } from "./$types";
    import type { ApiResponse, Litter } from "$lib/types";
    import { format } from "svelte-i18n";
    import { fetchLitter } from "$lib/api/litter";
    import { resdata } from "$lib/api";

    export let data: PageServerData;

    let names = data.data!;
    let litter: Promise<ApiResponse<Litter>> | undefined = undefined;

    function select(e: CustomEvent<number>) {
        litter = fetchLitter(e.detail);
    }
</script>

<h2 class="p-4 text-center text-4xl font-bold">{$format("nav.litters")}</h2>
<div class="flex flex-col md:flex-row md:px-[5%] lg:px-[25%]">
    <div
        class="relative flex flex-col border-t border-black md:w-1/4 dark:border-white"
    >
        <LitterList {names} on:select={select} />
    </div>
    <div class="border-t border-black md:w-3/4 dark:border-white">
        {#if litter !== undefined}
            {#await litter}
                <Loading text={$format("litter.display.loading")} />
            {:then litter}
                <LitterDisplay litter={resdata(litter).data} />
            {:catch}
                <Error message={$format("litter.display.error")} />
            {/await}
        {:else}
            <Empty text={$format("litter.inactive")} />
        {/if}
    </div>
</div>
