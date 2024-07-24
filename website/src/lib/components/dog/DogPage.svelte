<script lang="ts">
    import Permalink from "$lib/svg/Permalink.svelte";
    import Gallery from "$lib/components/Gallery.svelte";
    import { Breed } from "$lib/enums";
    import { format } from "svelte-i18n";

    $: australian = {
        title: $format(`nav.dog.${Breed.AUSTRALIAN}`),
        src: ["/aus-adult.webp", "/aus-puppy.webp", "/aus-draw.webp"],
        opts: { values: { breed: $format(`nav.dog.${Breed.AUSTRALIAN}`) } },
    };

    $: entlebuch = {
        title: $format(`nav.dog.${Breed.ENTLEBUCH}`),
        src: ["/ent-adult.webp", "/ent-puppy.webp", "/ent-draw.webp"],
        opts: { values: { breed: $format(`nav.dog.${Breed.ENTLEBUCH}`) } },
    };

    $: bernese = {
        title: $format(`nav.dog.${Breed.BERNESE}`),
        src: ["/ber-adult.webp", "/ber-puppy.webp", "/ber-draw.webp"],
        opts: { values: { breed: $format(`nav.dog.${Breed.BERNESE}`) } },
    };

    $: fci = $format("about.fci");
</script>

<div class="flex flex-col gap-8 pb-4 md:px-[5%] lg:px-[25%]">
    <section id={australian.title} class="flex flex-col">
        <div class="flex flex-row items-center justify-center gap-2">
            <h2 class="p-4 text-center text-4xl font-bold">
                {australian.title}
            </h2>
            <a href="#{australian.title}" class="">
                <Permalink />
            </a>
        </div>
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
        <p class="text-justify">{$format(`about.${Breed.AUSTRALIAN}`)}</p>
    </section>
    <section
        id={entlebuch.title}
        class="flex flex-col border-t border-black dark:border-white"
    >
        <div class="flex flex-row items-center justify-center gap-2">
            <h2 class="p-4 text-center text-4xl font-bold">
                {entlebuch.title}
            </h2>
            <a href="#{entlebuch.title}" class="">
                <Permalink />
            </a>
        </div>
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
        <p class="text-justify">{$format(`about.${Breed.ENTLEBUCH}`)}</p>
    </section>
    <section
        id={bernese.title}
        class="flex flex-col border-t border-black dark:border-white"
    >
        <div class="flex flex-row items-center justify-center gap-2">
            <h2 class="p-4 text-center text-4xl font-bold">
                {bernese.title}
            </h2>
            <a href="#{bernese.title}" class="">
                <Permalink />
            </a>
        </div>
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
        <p class="text-justify">{$format(`about.${Breed.BERNESE}`)}</p>
    </section>
</div>
