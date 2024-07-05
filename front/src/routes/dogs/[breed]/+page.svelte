<script lang="ts">
    import { Thumbnail, Loading, Empty, Error } from "$lib/components";
    import { Breed } from "$lib/types";
    import { page } from "$app/stores";
    import type { PageData } from "./$types";
    import { longBreed } from "$lib/util";

    export let data: PageData;

    $: breed = $page.params.breed as Breed;
</script>

<div class="flex flex-col items-center gap-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl">Our {longBreed(breed)}s</h2>
    <div class="flex w-full flex-col pb-4 md:flex-row">
        <div class="flex flex-col gap-4 md:w-1/3">
            <h3 class="p-2 text-center text-2xl">Male</h3>
            {#await data.male}
                <Loading text="Loading dogs..." />
            {:then dogs}
                {#if dogs.length > 0}
                    {#each dogs as { name, nickname, thumbnail }}
                        <Thumbnail
                            href={`${breed}/${nickname}`}
                            {name}
                            {nickname}
                            image={thumbnail}
                        />
                    {/each}
                {:else}
                    <Empty />
                {/if}
            {:catch}
                <Error message="Failed to load dogs, something went wrong" />
            {/await}
        </div>
        <div class="flex flex-col gap-4 md:w-1/3">
            <h3 class="p-2 text-center text-2xl">Female</h3>
            {#await data.female}
                <Loading text={"Loading dogs..."} />
            {:then dogs}
                {#if dogs.length > 0}
                    {#each dogs as { name, nickname, thumbnail }}
                        <Thumbnail
                            href={`${breed}/${nickname}`}
                            {name}
                            {nickname}
                            image={thumbnail}
                        />
                    {/each}
                {:else}
                    <Empty />
                {/if}
            {:catch}
                <Error message="Failed to load dogs, something went wrong" />
            {/await}
        </div>
        <div class="flex flex-col gap-4 md:w-1/3">
            <h3 class="p-2 text-center text-2xl">Retired</h3>
            {#await data.retired}
                <Loading text={"Loading dogs..."} />
            {:then dogs}
                {#if dogs.length > 0}
                    {#each dogs as { name, nickname, thumbnail }}
                        <Thumbnail
                            href={`${breed}/${nickname}`}
                            {name}
                            {nickname}
                            image={thumbnail}
                        />
                    {/each}
                {:else}
                    <Empty />
                {/if}
            {:catch}
                <Error message="Failed to load dogs, something went wrong" />
            {/await}
        </div>
    </div>
</div>
