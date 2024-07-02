<script>
    import { Person, Mail, Call, MapPin } from "$lib/components/svg";
</script>

<div class="flex flex-col md:px-44 lg:px-64">
    <h2 class="  p-4 text-center text-4xl">Contact</h2>
    <div class="flex flex-row gap-2 transition-colors duration-300">
        <Person />
        <p>Inge Aun</p>
    </div>
    <a
        href="mailto:ingeaun@gmail.com"
        title="ingeaun@gmail.com"
        class="group flex flex-row gap-2 transition-colors duration-300 hover:text-slate-500"
    >
        <Mail />
        <p>ingeaun@gmail.com</p>
    </a>
    <a
        href="tel:+37256226381"
        title="+372 5622 6381"
        class="group flex flex-row gap-2 transition-colors duration-300 hover:text-slate-500"
    >
        <Call />
        <p>+372 5622 6381</p>
    </a>
    <a
        class="group flex flex-row gap-2 transition-colors duration-300 hover:text-slate-500"
        href="https://www.google.com/maps/place/Kuusalu+Parish"
        title="Kuusalu parish"
        target="_blank"
    >
        <MapPin />
        <p>Kuusalu Parish, Harju County, Estonia</p>
    </a>
</div>
