<script lang="ts">
    import { Gender } from "$lib/types";
    import { page } from "$app/stores";
    import Thumbnail from "$lib/components/Thumbnail.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    // eslint-disable-next-line svelte/valid-compile
    $: shortBreed = $page.params.breed;
    $: dogs = data.dogs.filter(
        (d) => d.breed.split(" ")[0].toLowerCase() === shortBreed,
    );

    $: retired = dogs.filter((d) => !d.alive);
    $: alive = dogs.filter((d) => d.alive);
    $: male = alive.filter((d) => d.gender === Gender.MALE);
    $: female = alive.filter((d) => d.gender === Gender.FEMALE);
</script>

<div
    class="flex flex-col items-center gap-4 md:px-44 lg:px-64 dark:bg-black dark:text-white"
>
    <h2>Australian Shepherds</h2>
    <div class="flex flex-col md:flex-row">
        <div class="flex flex-col gap-4 md:w-1/2">
            <h3>Female</h3>
            {#each female as dog}
                <Thumbnail
                    href={`${shortBreed}/${dog.nickname.toLowerCase()}`}
                    {...dog}
                    {...dog.thumbnail}
                />
            {/each}
        </div>
        <div class="flex flex-col gap-4 md:w-1/2">
            <h3>Male</h3>
            {#each male as dog}
                <Thumbnail
                    href={`${shortBreed}/${dog.nickname.toLowerCase()}`}
                    {...dog}
                    {...dog.thumbnail}
                />
            {/each}
        </div>
    </div>
    <div class="flex flex-col gap-4 md:w-1/2">
        <h3>Retired</h3>
        {#each retired as dog}
            <Thumbnail
                href={`${shortBreed}/${dog.nickname.toLowerCase()}`}
                {...dog}
                {...dog.thumbnail}
            />
        {/each}
    </div>
</div>
