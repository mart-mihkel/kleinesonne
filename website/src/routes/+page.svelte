<script>
    import { Arrow } from "$lib/svg";
    import { Breed } from "$lib/types";
    import { format } from "svelte-i18n";
</script>

<a href="/puppies" class="flex justify-center p-4 text-4xl font-bold">
    {$format("about.puppies")}
</a>
<div class="flex flex-col md:h-[600px] md:flex-row">
    <a
        href="/puppies/{Breed.AUSTRALIAN}"
        class="group relative transition-all duration-700 ease-in-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-700 group-hover:brightness-100"
        >
            <enhanced:img
                class="size-full object-cover"
                src="/static/aus-puppy.avif"
                sizes="min(960, 50vw)"
                alt="7 week old Australian Shepherd puppies"
                fetchpriority="high"
                loading="eager"
            />
        </div>
        <p
            class="absolute left-1/2 top-1/2 flex -translate-x-1/2 flex-row items-center text-center text-xl font-semibold text-gray-300 transition-all duration-700 ease-in-out group-hover:opacity-0"
        >
            {$format(`nav.puppy.${Breed.AUSTRALIAN}`)}
            <Arrow />
        </p>
    </a>
    <a
        href="/puppies/{Breed.ENTLEBUCH}"
        class="group relative transition-all duration-700 ease-in-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-700 group-hover:brightness-100"
        >
            <enhanced:img
                class="size-full object-cover"
                src="/static/ent-puppy.avif"
                sizes="min(960, 50vw)"
                alt="Entlebuch Cattle Dog puppy"
                fetchpriority="high"
                loading="eager"
            />
        </div>
        <p
            class="absolute left-1/2 top-1/2 flex -translate-x-1/2 flex-row items-center text-center text-xl font-semibold text-gray-300 transition-all duration-700 ease-in-out group-hover:opacity-0"
        >
            {$format(`nav.puppy.${Breed.ENTLEBUCH}`)}
            <Arrow />
        </p>
    </a>
    <a
        href="/puppies/{Breed.BERNESE}"
        class="group relative transition-all duration-700 ease-in-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-700 group-hover:brightness-100"
        >
            <enhanced:img
                class="size-full object-cover"
                src="/static/ber-puppy.avif"
                sizes="min-width(960px, 50vw)"
                alt="Month old Bernese Mountain Dog puppies"
                fetchpriority="high"
                loading="eager"
            />
        </div>
        <p
            class="absolute left-1/2 top-1/2 flex -translate-x-1/2 flex-row items-center text-center text-xl font-semibold text-gray-300 transition-all duration-700 ease-in-out group-hover:opacity-0"
        >
            {$format(`nav.puppy.${Breed.BERNESE}`)}
            <Arrow />
        </p>
    </a>
</div>
<div class="flex flex-col gap-4 py-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">{$format("about.title")}</h2>
    <p class="text-justify">
        {$format("about.text")}
    </p>
</div>
