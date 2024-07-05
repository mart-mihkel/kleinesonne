<script lang="ts">
    import { Gallery } from "$lib/components";
    import { Breed, type Image } from "$lib/types";
    import { format } from "svelte-i18n";

    import ausAdult from "$lib/assets/aus-adult.jpg";
    import ausPuppy from "$lib/assets/aus-puppy.jpg";
    import ausDraw from "$lib/assets/aus-draw.jpg";
    const aus: Image[] = [
        { src: ausAdult, alt: "Adult merle Australian Shepherd" },
        { src: ausPuppy, alt: "7 week old Australian Shepherd puppies" },
        { src: ausDraw, alt: "Standard Australian Shepherd pencil drawing" },
    ];

    import entAdult from "$lib/assets/ent-adult.jpg";
    import entPuppy from "$lib/assets/ent-puppy.jpg";
    import entDraw from "$lib/assets/ent-draw.jpg";
    const ent: Image[] = [
        { src: entAdult, alt: "Adult Entlebuch Cattle Dog" },
        { src: entPuppy, alt: "Entlebuch Cattle Dog puppy" },
        { src: entDraw, alt: "Standard Entlebuch Cattle Dog pencil drawing" },
    ];

    import berAdult from "$lib/assets/ber-adult.jpg";
    import berPuppy from "$lib/assets/ber-puppy.jpg";
    import berDraw from "$lib/assets/ber-draw.jpg";
    const ber: Image[] = [
        { src: berAdult, alt: "Adult Bernese Mountain Dog" },
        { src: berPuppy, alt: "Month old Bernese Mountain Dog puppies" },
        { src: berDraw, alt: "Standard Bernsese Mountain Dog pencil drawing" },
    ];

    const fci = $format("breed.fci");
</script>

<div class="flex flex-col gap-8 pb-4 md:px-[5%] lg:px-[25%]">
    <div class="flex flex-col">
        <h2 class="p-4 text-center text-4xl">{$format("breed.aus.one")}</h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="/dogs/{Breed.AUSTRALIAN}"
            >
                {$format("dog.our_breed", {
                    values: { breed: $format("breed.aus.one") },
                })}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="https://www.fci.be/Nomenclature/Standards/342g01-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={aus} />
        <p class="text-justify">{$format("intro.aus")}</p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl">{$format("breed.ent.one")}</h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="/dogs/{Breed.ENTLEBUCH}"
            >
                {$format("dog.our_breed", {
                    values: { breed: $format("breed.ent.one") },
                })}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="https://www.fci.be/Nomenclature/Standards/047g02-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={ent} />
        <p class="text-justify">{$format("intro.ent")}</p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl">{$format("breed.ber.one")}</h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="/dogs/{Breed.BERNESE}"
            >
                {$format("dog.our_breed", {
                    values: { breed: $format("breed.ber.one") },
                })}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="https://www.fci.be/Nomenclature/Standards/045g02-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={ber} />
        <p class="text-justify">{$format("intro.ber")}</p>
    </div>
</div>
