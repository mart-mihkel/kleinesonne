<script>
    import { Person, Mail, Call, MapPin } from "$lib/components/svg";
</script>

<div class="flex justify-center md:px-[5%] lg:px-[25%]">
    <div class="flex flex-col gap-2 pb-4">
        <h2 class="p-4 text-center text-4xl">Contact</h2>
        <div class="flex flex-row gap-2 transition-colors duration-300">
            <Person />
            <p>Inge Aun</p>
        </div>
        <a
            href="mailto:ingeaun@gmail.com"
            class="group flex flex-row gap-2 transition-colors duration-300 hover:text-gray-500"
        >
            <Mail />
            <p>ingeaun@gmail.com</p>
        </a>
        <a
            href="tel:+37256226381"
            class="group flex flex-row gap-2 transition-colors duration-300 hover:text-gray-500"
        >
            <Call />
            <p>+372 5622 6381</p>
        </a>
        <a
            class="group flex flex-row gap-2 transition-colors duration-300 hover:text-gray-500"
            href="https://www.google.com/maps/place/Kuusalu+Parish"
            target="_blank"
        >
            <MapPin />
            <p>Kuusalu Parish, Harju County, Estonia</p>
        </a>
    </div>
</div>
