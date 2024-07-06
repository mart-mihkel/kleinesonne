<script lang="ts">
    import { Thumbnail, Loading, Empty, Error } from "$lib/components";
    import { page } from "$app/stores";
    import type { PageData } from "./$types";
    import { format } from "svelte-i18n";

    export let data: PageData;

    $: breed = $format(`breed.${$page.params.breed}.one`);
    $: options = { values: { breed } };
</script>

<div class="flex flex-col items-center gap-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">
        {$format("dog.our_breed", options)}
    </h2>
    <div class="flex w-full flex-col pb-4 md:flex-row">
        <div class="flex flex-col gap-4 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format("dog.male")}
            </h3>
            {#await data.male}
                <Loading text={$format("dog.loading.dogs")} />
            {:then dogs}
                {#if dogs.length > 0}
                    {#each dogs as { name, nickname, thumbnail }}
                        <Thumbnail
                            href={`${breed}/${nickname}`}
                            src={thumbnail}
                            {name}
                            {nickname}
                        />
                    {/each}
                {:else}
                    <Empty />
                {/if}
            {:catch}
                <Error message={$format("dog.error.dogs")} />
            {/await}
        </div>
        <div class="flex flex-col gap-4 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format("dog.female")}
            </h3>
            {#await data.female}
                <Loading text={$format("dog.loading.dogs")} />
            {:then dogs}
                {#if dogs.length > 0}
                    {#each dogs as { name, nickname, thumbnail }}
                        <Thumbnail
                            href={`${breed}/${nickname}`}
                            src={thumbnail}
                            {name}
                            {nickname}
                        />
                    {/each}
                {:else}
                    <Empty />
                {/if}
            {:catch}
                <Error message={$format("dog.error.dogs")} />
            {/await}
        </div>
        <div class="flex flex-col gap-4 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format("dog.retired")}
            </h3>
            {#await data.retired}
                <Loading text={$format("dog.loading.dogs")} />
            {:then dogs}
                {#if dogs.length > 0}
                    {#each dogs as { name, nickname, thumbnail }}
                        <Thumbnail
                            href={`${breed}/${nickname}`}
                            src={thumbnail}
                            {name}
                            {nickname}
                        />
                    {/each}
                {:else}
                    <Empty />
                {/if}
            {:catch}
                <Error message={$format("dog.error.dogs")} />
            {/await}
        </div>
    </div>
</div>
