<script lang="ts">
    import { Gallery } from "$lib/components";
    import { Breed } from "$lib/types";
    import { format } from "svelte-i18n";

    import ausAdult from "$lib/assets/aus-adult.avif";
    import ausPuppy from "$lib/assets/aus-puppy.avif";
    import ausDraw from "$lib/assets/aus-draw.avif";
    const aus = [ausAdult, ausPuppy, ausDraw];
    const ausAlt = [
        "Adult merle Australian Shepherd",
        "7 week old Australian Shepherd puppies",
        "Standard Australian Shepherd pencil drawing",
    ];

    import entAdult from "$lib/assets/ent-adult.avif";
    import entPuppy from "$lib/assets/ent-puppy.avif";
    import entDraw from "$lib/assets/ent-draw.avif";
    const ent = [entAdult, entPuppy, entDraw];
    const entAlt = [
        "Adult Entlebuch Cattle Dog",
        "Entlebuch Cattle Dog puppy",
        "Standard Entlebuch Cattle Dog pencil drawing",
    ];

    import berAdult from "$lib/assets/ber-adult.avif";
    import berPuppy from "$lib/assets/ber-puppy.avif";
    import berDraw from "$lib/assets/ber-draw.avif";
    const ber = [berAdult, berPuppy, berDraw];
    const berAlt = [
        "Adult Bernese Mountain Dog",
        "Month old Bernese Mountain Dog puppies",
        "Standard Bernsese Mountain Dog pencil drawing",
    ];

    const fci = $format("intro.fci");
</script>

<div class="flex flex-col gap-8 pb-4 md:px-[5%] lg:px-[25%]">
    <div class="flex flex-col">
        <h2 class="p-4 text-center text-4xl font-bold">
            {$format(`nav.dog.${Breed.AUSTRALIAN}`)}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.AUSTRALIAN}"
            >
                {$format("dog.our", {
                    values: { breed: $format(`nav.dog.${Breed.AUSTRALIAN}`) },
                })}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="https://www.fci.be/Nomenclature/Standards/342g01-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={aus} alts={ausAlt} />
        <p class="text-justify">{$format(`intro.${Breed.AUSTRALIAN}`)}</p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl font-bold">
            {$format(`nav.dog.${Breed.ENTLEBUCH}`)}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.ENTLEBUCH}"
            >
                {$format("dog.our", {
                    values: { breed: $format(`nav.dog.${Breed.ENTLEBUCH}`) },
                })}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="https://www.fci.be/Nomenclature/Standards/047g02-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={ent} alts={entAlt} />
        <p class="text-justify">{$format(`intro.${Breed.ENTLEBUCH}`)}</p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl font-bold">
            {$format(`nav.dog.${Breed.BERNESE}`)}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.BERNESE}"
            >
                {$format("dog.our", {
                    values: { breed: $format(`nav.dog.${Breed.BERNESE}`) },
                })}
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="https://www.fci.be/Nomenclature/Standards/045g02-en.pdf"
                target="_blank"
            >
                {fci}
            </a>
        </div>
        <Gallery images={ber} alts={berAlt} />
        <p class="text-justify">{$format(`intro.${Breed.BERNESE}`)}</p>
    </div>
</div>
