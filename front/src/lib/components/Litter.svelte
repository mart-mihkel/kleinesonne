<script lang="ts">
    import type { Litter } from "$lib/types";

    export let litter: Litter;

    const { name, puppies, parents } = litter;
</script>

<div class="flex flex-col items-center">
    <a href="/litters/{name}" title="Litter {name}" class="text-2xl">
        Litter {name}
    </a>
    <div class="flex flex-row flex-wrap">
        {#each puppies as { name, image, gender }}
            <div class="flex flex-col items-center p-4">
                <img
                    class="size-full object-cover"
                    loading="lazy"
                    src={image.src}
                    alt={image.alt}
                />
                <h4 class="text-center text-lg">{name}</h4>
                <p>{gender === 0 ? "Male" : "Female"}</p>
            </div>
        {/each}
    </div>
    <h3 class="text-center text-2xl">The parents</h3>
    <img src={parents.src} alt={parents.alt} />
    <p>info (vaksiinid asjad)</p>
    <div>fci link</div>
</div>
