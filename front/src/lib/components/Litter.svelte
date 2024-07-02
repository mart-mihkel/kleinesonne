<script lang="ts">
    import { Gender, Availability, type Litter } from "$lib/types";

    export let litter: Litter;
    export let onlyAvailable = false;

    const { name, parents } = litter;
    const puppies = onlyAvailable
        ? litter.puppies.filter(
              (p) => p.availability !== Availability.UNAVAILABLE,
          )
        : litter.puppies;
</script>

<div
    class="flex flex-col items-center gap-2 border-t border-t-black dark:border-t-white"
>
    <a href="/litters/{name}" title="Litter {name}" class="text-2xl">
        Litter {name}
    </a>
    <div class="flex flex-row flex-wrap justify-center">
        {#each puppies as { name, image, gender, availability }}
            <div class="flex flex-col items-center p-4">
                <img
                    class="size-full object-cover"
                    loading="lazy"
                    src={image.src}
                    alt={image.alt}
                />
                <h4 class="text-center text-lg">{name}</h4>
                <p>
                    Available {gender === Gender.MALE ? "male" : "female"}
                    {#if availability === Availability.CO_OWNERSHIP}
                        for co-ownership
                    {/if}
                </p>
            </div>
        {/each}
    </div>
    <h3 class="text-center text-2xl">The parents</h3>
    <img src={parents.src} alt={parents.alt} />
    <p>
        The puppies will leave for their new homes healthy, having been checked
        by a veterinarian, vaccinated and protected against parasites.
    </p>
    <p>The puppies are chipped, FCI registered and have an euro passport.</p>
</div>
