<script lang="ts">
    import Call from "$lib/svg/Call.svelte";
    import Facebook from "$lib/svg/Facebook.svelte";
    import Mail from "$lib/svg/Mail.svelte";
    import MapPin from "$lib/svg/MapPin.svelte";
</script>

<footer
    class="flex flex-row justify-center gap-2 border-t border-black p-8 md:px-[5%] lg:px-[25%] dark:border-white dark:bg-black"
>
    <a href="tel:+37256226381" title="+372 5622 6381">
        <Call />
    </a>
    <a href="mailto:ingeaun@gmail.com" title="ingeaun@gmail.com">
        <Mail />
    </a>
    <a href="https://www.google.com/maps/place/Kuusalu+Parish" target="_blank">
        <MapPin />
    </a>
    <a
        href="https://www.facebook.com/p/Kleine-Sonne-100064871220699/"
        target="_blank"
    >
        <Facebook />
    </a>
</footer>
