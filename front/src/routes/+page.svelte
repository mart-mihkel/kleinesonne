<script>
    import { Breed } from "$lib/types";
    import { format } from "svelte-i18n";
    import aus from "$lib/assets/aus-puppy.jpg";
    import ent from "$lib/assets/ent-puppy.jpg";
    import ber from "$lib/assets/ber-puppy.jpg";
</script>

<a href="/puppies" class="flex justify-center p-4 text-4xl">
    {$format("puppies.available")}
</a>
<div class="flex flex-col md:h-[600px] md:flex-row">
    <a
        href="/puppies/{Breed.AUSTRALIAN}"
        class="group relative transition-all duration-500 ease-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-500 group-hover:brightness-100"
        >
            <img
                class="size-full object-cover"
                src={aus}
                alt="7 week old Australian Shepherd puppies"
                fetchpriority="high"
                loading="eager"
            />
        </div>
        <p
            class="absolute bottom-1 left-1/2 -translate-x-1/2 text-center font-sans text-xl text-white transition-all duration-500 ease-in-out group-hover:opacity-0"
        >
            {$format("puppies.aus")}
        </p>
    </a>
    <a
        href="/puppies/{Breed.ENTLEBUCH}"
        class="group relative transition-all duration-500 ease-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-500 group-hover:brightness-100"
        >
            <img
                class="size-full object-cover"
                src={ent}
                alt="Entlebuch Cattle Dog puppy"
                fetchpriority="high"
                loading="eager"
            />
        </div>
        <p
            class="absolute bottom-1 left-1/2 -translate-x-1/2 text-center font-sans text-xl text-white transition-all duration-500 ease-in-out group-hover:opacity-0"
        >
            {$format("puppies.ent")}
        </p>
    </a>
    <a
        href="/puppies/{Breed.BERNESE}"
        class="group relative transition-all duration-500 ease-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-500 group-hover:brightness-100"
        >
            <img
                class="size-full object-cover"
                src={ber}
                alt="Month old Bernese Mountain Dog puppies"
                fetchpriority="high"
                loading="eager"
            />
        </div>
        <p
            class="absolute bottom-1 left-1/2 -translate-x-1/2 text-center font-sans text-xl text-white transition-all duration-500 ease-in-out group-hover:opacity-0"
        >
            {$format("puppies.ber")}
        </p>
    </a>
</div>
<div class="flex flex-col gap-4 py-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl">{$format("about.title")}</h2>
    <p class="text-justify">
        {$format("about.text")}
    </p>
</div>
