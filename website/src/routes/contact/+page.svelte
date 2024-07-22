<script>
    import Call from "$lib/svg/Call.svelte";
    import Mail from "$lib/svg/Mail.svelte";
    import MapPin from "$lib/svg/MapPin.svelte";
    import Person from "$lib/svg/Person.svelte";
    import { format } from "svelte-i18n";
</script>

<div class="flex justify-center md:px-[5%] lg:px-[25%]">
    <div class="flex flex-col gap-2 pb-4">
        <h2 class="p-4 text-center text-4xl font-bold">
            {$format("nav.contact")}
        </h2>
        <div class="flex flex-row gap-2 transition-colors duration-300">
            <Person />
            <p class="font-semibold">Inge Aun</p>
        </div>
        <a
            href="mailto:ingeaun@gmail.com"
            class="group flex flex-row gap-2 transition-colors duration-300 hover:text-gray-500"
        >
            <Mail />
            <p class="font-semibold">ingeaun@gmail.com</p>
        </a>
        <a
            href="tel:+37256226381"
            class="group flex flex-row gap-2 transition-colors duration-300 hover:text-gray-500"
        >
            <Call />
            <p class="font-semibold">+372 5622 6381</p>
        </a>
        <a
            class="group flex flex-row gap-2 transition-colors duration-300 hover:text-gray-500"
            href="https://www.google.com/maps/place/Kuusalu+Parish"
            target="_blank"
        >
            <MapPin />
            <p class="font-semibold">{$format("contact.location")}</p>
        </a>
    </div>
</div>
