<script lang="ts">
    import Thumbnail from "../Thumbnail.svelte";
    import Empty from "../notice/Empty.svelte";
    import Error from "../notice/Error.svelte";
    import type { Dog } from "$lib/types";
    import { Breed } from "$lib/enums";
    import { format } from "svelte-i18n";

    export let dogs: Dog[] | undefined;
    export let breed: Breed;
</script>

{#if dogs === undefined}
    <Error message={$format("dog.error.dogs")} />
{:else if dogs.length === 0}
    <Empty />
{:else}
    {#each dogs as { id, name, nickname, thumbnail }}
        <Thumbnail
            href={`/dogs/${breed}/${id}`}
            src={thumbnail}
            {name}
            {nickname}
        />
    {/each}
{/if}
