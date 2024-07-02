<script lang="ts">
    import { Gender, Availability, type Litter } from "$lib/types";
    import Thumbnail from "./Thumbnail.svelte";

    export let litter: Litter;
    export let ad = false;

    const { name, parents } = litter;
    const puppies = !ad
        ? litter.puppies
        : litter.puppies.filter(
              (p) => p.availability !== Availability.UNAVAILABLE,
          );
</script>

<div
    class="flex flex-col items-center gap-2 border-t border-t-black py-4 dark:border-t-white"
>
    <a href="/litters/{name}" class="text-2xl">
        Litter {name}
    </a>
    <div class="flex flex-row flex-wrap justify-center">
        {#each puppies as { name, gender, image, availability }}
            <div class="flex w-full flex-col items-center py-4 md:w-96 md:px-4">
                <Thumbnail {name} src={image.src} alt={image.alt} />
                <p>
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
    <h3 class="text-center text-2xl">The parents</h3>
    <img src={parents.src} alt={parents.alt} />
    {#if ad}
        <p>
            The puppies will leave for their new homes healthy, having been
            checked by a veterinarian, vaccinated and protected against
            parasites.
        </p>
        <p>
            The puppies are chipped, FCI registered and have an euro passport.
        </p>
    {/if}
</div>
