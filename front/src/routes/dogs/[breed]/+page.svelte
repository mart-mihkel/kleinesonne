<script lang="ts">
    import { Thumbnail } from "$lib/components";
    import { Breed, Gender } from "$lib/types";
    import { page } from "$app/stores";
    import type { PageData } from "./$types";
    import { longBreed } from "$lib/util";

    export let data: PageData;

    // eslint-disable-next-line svelte/valid-compile
    $: breed = $page.params.breed as Breed;
    $: dogs = data.dogs.filter((d) => d.breed === breed);

    $: alive = dogs.filter((d) => d.alive);
    $: segregated = [
        { t: "Male", dogs: alive.filter((d) => d.gender === Gender.MALE) },
        { t: "Female", dogs: alive.filter((d) => d.gender === Gender.FEMALE) },
        { t: "Retired", dogs: dogs.filter((d) => !d.alive) },
    ];
</script>

<div class="flex flex-col items-center gap-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl">Our {longBreed(breed)}s</h2>
    <div class="flex w-full flex-col pb-4 md:flex-row">
        {#each segregated as { t, dogs }}
            <div class="flex flex-col gap-4 md:w-1/3">
                <h3 class="p-2 text-center text-2xl">{t}</h3>
                {#each dogs as { name, nickname, thumbnail }}
                    <Thumbnail
                        href={`${breed}/${nickname}`}
                        {name}
                        {nickname}
                        src={thumbnail.src}
                        alt={thumbnail.alt}
                    />
                {/each}
            </div>
        {/each}
    </div>
</div>
