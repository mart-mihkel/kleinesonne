<script>
    import Arrow from "$lib/svg/Arrow.svelte";
    import { Breed } from "$lib/enums";
    import { format } from "svelte-i18n";
    import DogPage from "$lib/components/dog/DogPage.svelte";
</script>

<a
    href="/puppies"
    class="flex justify-center text-nowrap p-4 text-4xl font-bold"
>
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
            <img
                class="size-full object-cover"
                src="/aus.webp"
                alt=""
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
            <img
                class="size-full object-cover"
                src="/ent.webp"
                alt=""
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
            <img
                class="size-full object-cover"
                src="/ber.webp"
                alt=""
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
<DogPage />
