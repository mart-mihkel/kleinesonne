<script lang="ts">
    import { Breed } from "$lib/enums";
    import { Gallery } from "$lib/components";
    import { format } from "svelte-i18n";

    import aadult from "$lib/assets/aus-adult.webp";
    import apuppy from "$lib/assets/aus-puppy.webp";
    import adraw from "$lib/assets/aus-draw.webp";
    const australian = {
        title: $format(`nav.dog.${Breed.AUSTRALIAN}`),
        src: [aadult, apuppy, adraw],
        opts: { values: { breed: $format(`nav.dog.${Breed.AUSTRALIAN}`) } },
    };

    import eadult from "$lib/assets/ent-adult.webp";
    import epuppy from "$lib/assets/ent-puppy.webp";
    import edraw from "$lib/assets/ent-draw.webp";
    const entlebuch = {
        title: $format(`nav.dog.${Breed.ENTLEBUCH}`),
        src: [eadult, epuppy, edraw],
        opts: { values: { breed: $format(`nav.dog.${Breed.ENTLEBUCH}`) } },
    };

    import badult from "$lib/assets/ber-adult.webp";
    import bpuppy from "$lib/assets/ber-puppy.webp";
    import bdraw from "$lib/assets/ber-draw.webp";
    const bernese = {
        title: $format(`nav.dog.${Breed.BERNESE}`),
        src: [badult, bpuppy, bdraw],
        opts: { values: { breed: $format(`nav.dog.${Breed.BERNESE}`) } },
    };

    const fci = $format("intro.fci");
</script>

<div class="flex flex-col gap-8 pb-4 md:px-[5%] lg:px-[25%]">
    <section id={australian.title} class="flex flex-col">
        <h2 class="p-4 text-center text-4xl font-bold">
            {australian.title}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.AUSTRALIAN}"
            >
                {$format("dog.our", australian.opts)}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="https://www.fci.be/Nomenclature/Standards/342g01-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={australian.src} full />
        <p class="text-justify">{$format(`intro.${Breed.AUSTRALIAN}`)}</p>
    </section>
    <section
        id={entlebuch.title}
        class="flex flex-col border-t border-black dark:border-white"
    >
        <h2 class="p-4 text-center text-4xl font-bold">
            {entlebuch.title}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.ENTLEBUCH}"
            >
                {$format("dog.our", entlebuch.opts)}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="https://www.fci.be/Nomenclature/Standards/047g02-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={entlebuch.src} full />
        <p class="text-justify">{$format(`intro.${Breed.ENTLEBUCH}`)}</p>
    </section>
    <section
        id={bernese.title}
        class="flex flex-col border-t border-black dark:border-white"
    >
        <h2 class="p-4 text-center text-4xl font-bold">
            {bernese.title}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.BERNESE}"
            >
                {$format("dog.our", bernese.opts)}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="https://www.fci.be/Nomenclature/Standards/045g02-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={bernese.src} full />
        <p class="text-justify">{$format(`intro.${Breed.BERNESE}`)}</p>
    </section>
</div>
