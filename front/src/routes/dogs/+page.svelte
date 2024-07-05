<script lang="ts">
    import { Gallery } from "$lib/components";
    import { Breed, type Image } from "$lib/types";

    import ausAdult from "$lib/assets/aus-adult.jpg";
    import ausPuppy from "$lib/assets/aus-puppy.jpg";
    import ausDraw from "$lib/assets/aus-draw.jpg";
    const aus: Image[] = [
        { src: ausAdult, alt: "Adult merle Australian Shepherd" },
        { src: ausPuppy, alt: "7 week old Australian Shepherd puppies" },
        { src: ausDraw, alt: "Standard Australian Shepherd pencil drawing" },
    ];

    import entAdult from "$lib/assets/ent-adult.jpg";
    import entPuppy from "$lib/assets/ent-puppy.jpg";
    import entDraw from "$lib/assets/ent-draw.jpg";
    const ent: Image[] = [
        { src: entAdult, alt: "Adult Entlebuch Cattle Dog" },
        { src: entPuppy, alt: "Entlebuch Cattle Dog puppy" },
        { src: entDraw, alt: "Standard Entlebuch Cattle Dog pencil drawing" },
    ];

    import berAdult from "$lib/assets/ber-adult.jpg";
    import berPuppy from "$lib/assets/ber-puppy.jpg";
    import berDraw from "$lib/assets/ber-draw.jpg";
    const ber: Image[] = [
        { src: berAdult, alt: "Adult Bernese Mountain Dog" },
        { src: berPuppy, alt: "Month old Bernese Mountain Dog puppies" },
        { src: berDraw, alt: "Standard Bernsese Mountain Dog pencil drawing" },
    ];
</script>

<div class="flex flex-col gap-8 pb-4 md:px-[5%] lg:px-[25%]">
    <div class="flex flex-col">
        <h2 class="p-4 text-center text-4xl">Australian Shepherd</h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="/dogs/{Breed.AUSTRALIAN}"
            >
                Our Australian Shepherds
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="https://www.fci.be/Nomenclature/Standards/342g01-en.pdf"
                target="_blank"
            >
                FCI Breed Standard
            </a>
        </div>
        <Gallery images={aus} />
        <p class="text-justify">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam
            egestas ultrices tincidunt. Ut vehicula tellus rhoncus interdum
            elementum. Mauris enim nisl, posuere nec diam vitae, posuere finibus
            lacus. Curabitur eu tristique lectus. Cras a nunc ut lorem sodales
            fermentum et eget est. Donec non tempor ante. Pellentesque accumsan
            tortor in quam fermentum molestie. Proin sed ligula tristique,
            scelerisque nunc at, lobortis dolor. Quisque efficitur ornare mi, ut
            accumsan libero egestas non. Curabitur quis lacus dolor. Vestibulum
            a arcu nisi. In luctus blandit mauris eu tempor. Vivamus convallis
            sodales tortor. Vivamus quis tortor sodales, malesuada purus eget,
            tincidunt diam. Nam sagittis, massa eu ullamcorper faucibus, eros ex
            convallis ante, at ultricies lacus ex quis metus. Aliquam
            ullamcorper sapien sed tellus aliquet, vel tincidunt eros maximus.
            Nam a orci sollicitudin, sodales nulla ac, consectetur mi. Etiam
            commodo arcu et euismod vulputate. Aliquam erat volutpat. Etiam
            volutpat, sapien vel dapibus imperdiet, diam nibh aliquet risus,
            eget tempor arcu leo non neque. Proin viverra iaculis orci. Nulla
            lacinia orci metus, condimentum ullamcorper turpis condimentum eu.
            Vivamus vel est at metus semper dapibus ac quis est. Suspendisse
            diam purus, ultricies nec euismod imperdiet, vehicula sit amet ex.
            Integer ut pellentesque purus, at tristique tortor. Nunc consectetur
            urna in rhoncus auctor. Vestibulum pharetra mi vitae tempor
            porttitor. Pellentesque bibendum quam sed ex euismod pretium. Nunc
            consectetur mauris sit amet erat hendrerit gravida. Aliquam non
            porta mauris, sed facilisis quam. Donec faucibus diam at lacus
            luctus faucibus. Praesent eros tellus, lobortis id libero quis,
            suscipit malesuada nunc. Mauris eget imperdiet odio. Morbi pulvinar
            tincidunt fringilla. Vestibulum ante ipsum primis in faucibus orci
            luctus et ultrices posuere cubilia curae; Aliquam non ex dui.
            Maecenas lacinia tellus id diam hendrerit, quis accumsan libero
            maximus. Duis sit amet luctus tellus. Curabitur dui nibh, interdum a
            porttitor a, hendrerit commodo ex. Vestibulum tempor nulla mollis,
            fermentum est quis, mattis diam. Class aptent taciti sociosqu ad
            litora torquent per conubia nostra, per inceptos himenaeos. Proin eu
            mauris non ante ullamcorper ultrices. Cras sed dui sodales, cursus
            orci id, fermentum neque. Proin bibendum id ipsum et eleifend.
        </p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl">Entlebuch Cattle Dog</h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="/dogs/{Breed.ENTLEBUCH}"
            >
                Our Enltebucher Cattle Dogs
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="https://www.fci.be/Nomenclature/Standards/047g02-en.pdf"
                target="_blank"
            >
                FCI Breed Standard
            </a>
        </div>
        <Gallery images={ent} />
        <p class="text-justify">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam
            egestas ultrices tincidunt. Ut vehicula tellus rhoncus interdum
            elementum. Mauris enim nisl, posuere nec diam vitae, posuere finibus
            lacus. Curabitur eu tristique lectus. Cras a nunc ut lorem sodales
            fermentum et eget est. Donec non tempor ante. Pellentesque accumsan
            tortor in quam fermentum molestie. Proin sed ligula tristique,
            scelerisque nunc at, lobortis dolor. Quisque efficitur ornare mi, ut
            accumsan libero egestas non. Curabitur quis lacus dolor. Vestibulum
            a arcu nisi. In luctus blandit mauris eu tempor. Vivamus convallis
            sodales tortor. Vivamus quis tortor sodales, malesuada purus eget,
            tincidunt diam. Nam sagittis, massa eu ullamcorper faucibus, eros ex
            convallis ante, at ultricies lacus ex quis metus. Aliquam
            ullamcorper sapien sed tellus aliquet, vel tincidunt eros maximus.
            Nam a orci sollicitudin, sodales nulla ac, consectetur mi. Etiam
            commodo arcu et euismod vulputate. Aliquam erat volutpat. Etiam
            volutpat, sapien vel dapibus imperdiet, diam nibh aliquet risus,
            eget tempor arcu leo non neque. Proin viverra iaculis orci. Nulla
            lacinia orci metus, condimentum ullamcorper turpis condimentum eu.
            Vivamus vel est at metus semper dapibus ac quis est. Suspendisse
            diam purus, ultricies nec euismod imperdiet, vehicula sit amet ex.
            Integer ut pellentesque purus, at tristique tortor. Nunc consectetur
            urna in rhoncus auctor. Vestibulum pharetra mi vitae tempor
            porttitor. Pellentesque bibendum quam sed ex euismod pretium. Nunc
            consectetur mauris sit amet erat hendrerit gravida. Aliquam non
            porta mauris, sed facilisis quam. Donec faucibus diam at lacus
            luctus faucibus. Praesent eros tellus, lobortis id libero quis,
            suscipit malesuada nunc. Mauris eget imperdiet odio. Morbi pulvinar
            tincidunt fringilla. Vestibulum ante ipsum primis in faucibus orci
            luctus et ultrices posuere cubilia curae; Aliquam non ex dui.
            Maecenas lacinia tellus id diam hendrerit, quis accumsan libero
            maximus. Duis sit amet luctus tellus. Curabitur dui nibh, interdum a
            porttitor a, hendrerit commodo ex. Vestibulum tempor nulla mollis,
            fermentum est quis, mattis diam. Class aptent taciti sociosqu ad
            litora torquent per conubia nostra, per inceptos himenaeos. Proin eu
            mauris non ante ullamcorper ultrices. Cras sed dui sodales, cursus
            orci id, fermentum neque. Proin bibendum id ipsum et eleifend.
        </p>
    </div>
    <div class="flex flex-col border-t border-black dark:border-white">
        <h2 class="p-4 text-center text-4xl">Bernese Mounatin Dog</h2>
        <div class="flex flex-row justify-center gap-16 p-4">
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="/dogs/{Breed.BERNESE}"
            >
                Our Bernese Mountain Dogs
            </a>
            <a
                class="rounded-md border-2 border-black px-4 py-2 text-center transition-colors duration-300 ease-out hover:bg-black hover:text-white dark:border-white dark:hover:bg-white dark:hover:text-black"
                href="https://www.fci.be/Nomenclature/Standards/045g02-en.pdf"
                target="_blank"
            >
                FCI Breed Standard
            </a>
        </div>
        <Gallery images={ber} />
        <p class="text-justify">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam
            egestas ultrices tincidunt. Ut vehicula tellus rhoncus interdum
            elementum. Mauris enim nisl, posuere nec diam vitae, posuere finibus
            lacus. Curabitur eu tristique lectus. Cras a nunc ut lorem sodales
            fermentum et eget est. Donec non tempor ante. Pellentesque accumsan
            tortor in quam fermentum molestie. Proin sed ligula tristique,
            scelerisque nunc at, lobortis dolor. Quisque efficitur ornare mi, ut
            accumsan libero egestas non. Curabitur quis lacus dolor. Vestibulum
            a arcu nisi. In luctus blandit mauris eu tempor. Vivamus convallis
            sodales tortor. Vivamus quis tortor sodales, malesuada purus eget,
            tincidunt diam. Nam sagittis, massa eu ullamcorper faucibus, eros ex
            convallis ante, at ultricies lacus ex quis metus. Aliquam
            ullamcorper sapien sed tellus aliquet, vel tincidunt eros maximus.
            Nam a orci sollicitudin, sodales nulla ac, consectetur mi. Etiam
            commodo arcu et euismod vulputate. Aliquam erat volutpat. Etiam
            volutpat, sapien vel dapibus imperdiet, diam nibh aliquet risus,
            eget tempor arcu leo non neque. Proin viverra iaculis orci. Nulla
            lacinia orci metus, condimentum ullamcorper turpis condimentum eu.
            Vivamus vel est at metus semper dapibus ac quis est. Suspendisse
            diam purus, ultricies nec euismod imperdiet, vehicula sit amet ex.
            Integer ut pellentesque purus, at tristique tortor. Nunc consectetur
            urna in rhoncus auctor. Vestibulum pharetra mi vitae tempor
            porttitor. Pellentesque bibendum quam sed ex euismod pretium. Nunc
            consectetur mauris sit amet erat hendrerit gravida. Aliquam non
            porta mauris, sed facilisis quam. Donec faucibus diam at lacus
            luctus faucibus. Praesent eros tellus, lobortis id libero quis,
            suscipit malesuada nunc. Mauris eget imperdiet odio. Morbi pulvinar
            tincidunt fringilla. Vestibulum ante ipsum primis in faucibus orci
            luctus et ultrices posuere cubilia curae; Aliquam non ex dui.
            Maecenas lacinia tellus id diam hendrerit, quis accumsan libero
            maximus. Duis sit amet luctus tellus. Curabitur dui nibh, interdum a
            porttitor a, hendrerit commodo ex. Vestibulum tempor nulla mollis,
            fermentum est quis, mattis diam. Class aptent taciti sociosqu ad
            litora torquent per conubia nostra, per inceptos himenaeos. Proin eu
            mauris non ante ullamcorper ultrices. Cras sed dui sodales, cursus
            orci id, fermentum neque. Proin bibendum id ipsum et eleifend.
        </p>
    </div>
</div>
