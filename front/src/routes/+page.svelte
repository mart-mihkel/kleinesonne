<script>
    import { Breed } from "$lib/types";
</script>

<a href="/puppies" class="flex justify-center p-4 text-4xl">
    Puppies available
</a>
<div class="flex flex-col md:h-[600px] md:flex-row">
    <a
        href="/puppies/{Breed.AUSTRALIAN}"
        title="Australian Shpeherd puppies"
        class="group relative transition-all duration-500 ease-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-500 group-hover:brightness-100"
        >
            <img
                class="size-full object-cover"
                src="/rand.webp"
                alt="australian shepherd"
            />
        </div>
        <p
            class="absolute bottom-1 left-1/2 -translate-x-1/2 text-center font-sans text-xl text-white transition-all duration-500 ease-out group-hover:text-2xl"
        >
            Australian Shepeherd puppies
        </p>
    </a>
    <a
        href="/puppies/{Breed.ENTLEBUCHER}"
        title="Entlebucher Mountain dog puppies"
        class="group relative transition-all duration-500 ease-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-500 group-hover:brightness-100"
        >
            <img
                class="size-full object-cover"
                src="/viisakas.webp"
                alt="entlebuch"
            />
        </div>
        <p
            class="absolute bottom-1 left-1/2 -translate-x-1/2 text-center font-sans text-xl text-white transition-all duration-500 ease-out group-hover:text-2xl"
        >
            Entlebucher Mountain Dog puppies
        </p>
    </a>
    <a
        href="/puppies/{Breed.BERNESE}"
        title="Bernese Mountain dog puppies"
        class="group relative transition-all duration-500 ease-out md:w-1/3 hover:md:w-1/2"
    >
        <div
            class="size-full brightness-50 transition-all duration-500 group-hover:brightness-100"
        >
            <img
                class="size-full object-cover"
                src="/usin.webp"
                alt="bernese"
            />
        </div>
        <p
            class="absolute bottom-1 left-1/2 -translate-x-1/2 text-center font-sans text-xl text-white transition-all duration-500 ease-out group-hover:text-2xl"
        >
            Bernese Mountain Dog puppies
        </p>
    </a>
</div>
<div class="flex flex-col gap-4 py-4 md:px-44 lg:px-64">
    <h2 class="p-4 text-center text-4xl">About</h2>
    <p class="text-justify">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus
        commodo leo aliquet massa viverra dapibus. Aliquam at viverra tortor.
        Vestibulum eros sapien, tempor in elit a, faucibus vehicula ipsum.
        Aliquam vel dapibus massa. Curabitur sed porta eros. Pellentesque quis
        ligula vel risus congue commodo nec ut urna. Fusce ut nibh neque.
        Aliquam ac purus facilisis, elementum felis luctus, tincidunt quam.
        Nullam id diam a mi lobortis sodales aliquet et justo. Donec tristique
        lectus sit amet porttitor pharetra. Proin a luctus velit. Mauris
        malesuada, eros eu maximus ultricies, massa dolor hendrerit dui, id
        malesuada turpis purus at tortor. Donec feugiat tempor ex vitae
        pellentesque.
    </p>
    <p class="text-justify">
        Ut non arcu in massa faucibus aliquam at nec risus. Fusce vulputate
        lacinia aliquet. Proin viverra dolor mauris, at ultricies neque
        ullamcorper nec. Fusce egestas mi eu nunc lacinia, imperdiet gravida
        felis auctor. Vestibulum id dapibus elit, ac molestie risus. In hac
        habitasse platea dictumst. Duis in maximus odio. Nam nec purus tellus.
        Cras vulputate purus sed neque eleifend, eget finibus massa vehicula.
        Nam eleifend metus nec erat volutpat vehicula. Integer id massa sit amet
        sem sagittis ultrices. Proin a sodales massa, in dictum ex. Duis viverra
        elit eu leo pellentesque tincidunt. Pellentesque a neque eget tortor
        sagittis venenatis vitae vel odio. Maecenas sit amet ipsum eu orci
        tristique pretium. Etiam dictum sagittis risus, non lacinia urna
        vestibulum et.
    </p>
    <p class="text-justify">
        Phasellus facilisis euismod lectus eu tincidunt. Aenean in diam ac
        lectus molestie venenatis ac et sapien. Suspendisse justo elit,
        venenatis eget consectetur vitae, condimentum ac risus. Nullam ut dolor
        est. Cras placerat ultricies libero mollis sollicitudin. Quisque vel
        velit est. Pellentesque nisl velit, varius vel euismod quis, tempor vel
        ex. Nulla ut faucibus nisi. Phasellus pharetra arcu odio, at rutrum eros
        auctor ac.
    </p>
</div>
