<script lang="ts">
    import { Gallery } from "$lib/components";
    import { Breed } from "$lib/types";
    import { format } from "svelte-i18n";

    import ausAdult from "$lib/assets/aus-adult.jpg?as=run";
    import ausPuppy from "$lib/assets/aus-puppy.jpg?as=run";
    import ausDraw from "$lib/assets/aus-draw.jpg?as=run";
    const aus = [ausAdult, ausPuppy, ausDraw];
    const ausAlt = [
        "Adult merle Australian Shepherd",
        "7 week old Australian Shepherd puppies",
        "Standard Australian Shepherd pencil drawing",
    ];

    import entAdult from "$lib/assets/ent-adult.jpg?as=run";
    import entPuppy from "$lib/assets/ent-puppy.jpg?as=run";
    import entDraw from "$lib/assets/ent-draw.jpg?as=run";
    const ent = [entAdult, entPuppy, entDraw];
    const entAlt = [
        "Adult Entlebuch Cattle Dog",
        "Entlebuch Cattle Dog puppy",
        "Standard Entlebuch Cattle Dog pencil drawing",
    ];

    import berAdult from "$lib/assets/ber-adult.jpg?as=run";
    import berPuppy from "$lib/assets/ber-puppy.jpg?as=run";
    import berDraw from "$lib/assets/ber-draw.jpg?as=run";
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
            {$format("breed.aus.one")}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.AUSTRALIAN}"
            >
                {$format("dog.our_breed", {
                    values: { breed: $format("breed.aus.one") },
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
        <p class="text-justify">{$format("intro.aus")}</p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl font-bold">
            {$format("breed.ent.one")}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.ENTLEBUCH}"
            >
                {$format("dog.our_breed", {
                    values: { breed: $format("breed.ent.one") },
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
        <p class="text-justify">{$format("intro.ent")}</p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl font-bold">
            {$format("breed.ber.one")}
        </h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                href="/dogs/{Breed.BERNESE}"
            >
                {$format("dog.our_breed", {
                    values: { breed: $format("breed.ber.one") },
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
        <p class="text-justify">{$format("intro.ber")}</p>
    </div>
</div>
