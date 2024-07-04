<script lang="ts">
    import { Gender, Availability, type Litter } from "$lib/types";
    import Thumbnail from "./Thumbnail.svelte";

    export let litter: Litter;

    const { name, parents, puppies } = litter;
</script>

<div
    class="flex flex-col items-center gap-2 border-t border-t-black py-4 dark:border-t-white"
>
    <a href="/litters/{name}" class="text-2xl">
        Litter {name}
    </a>
    <div class="flex flex-row flex-wrap">
        {#each puppies as { name, gender, image, availability }}
            <div class="flex w-1/2 flex-col items-center p-1 lg:w-1/3">
                <Thumbnail {name} src={image.src} alt={image.alt} />
                <p class="text-center">
                    {#if availability === Availability.AVAILABLE}
                        Available {gender === Gender.MALE ? "male" : "female"}
                    {:else if availability === Availability.CO_OWNERSHIP}
                        {gender === Gender.MALE ? "Male" : "Female"} available for
                        co-ownership
                    {:else}
                        {gender === Gender.MALE ? "Male" : "Female"}
                    {/if}
                </p>
            </div>
        {/each}
    </div>
    <h3 class="text-center text-2xl">Parents</h3>
    <img src={parents.src} alt={parents.alt} />
</div>
