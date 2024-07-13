<script lang="ts">
    import type { Breed, Dog } from "$lib/types";
    import { Empty, Error, Loading, Thumbnail } from "$lib/components";
    import { format } from "svelte-i18n";

    export let promise: Promise<Dog[]>;
    export let breed: Breed;
</script>

{#await promise}
    <Loading text={$format("dog.loading.dogs")} />
{:then dogs}
    {#if dogs.length > 0}
        {#each dogs as { id, name, nickname, thumbnail }}
            <Thumbnail
                href={`/dogs/${breed}/${id}`}
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
