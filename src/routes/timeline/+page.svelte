<script>
    import { onMount } from 'svelte';
    import { pan } from 'svelte-gestures';

    let frameNumber = 0;
    let swipePosition = 0;
    let imageSrc = '';
    let debounceTimer;

    function debounce(func, delay) {
        return function(...args) {
            clearTimeout(debounceTimer);
            debounceTimer = setTimeout(() => func.apply(this, args), delay);
        };
    }

    const loadImage = debounce(() => {
        imageSrc = `http://localhost:3030/get_frame/${frameNumber}`;
        console.log(imageSrc);
    }, 100);

    function updateFrame() {
        const newFrame = Math.round(swipePosition);
        if (newFrame !== frameNumber) {
            frameNumber = newFrame;
            loadImage();
        }
    }

    function handlePan(event) {
        const { deltaX } = event;
        swipePosition = Math.min(Math.max(0, swipePosition + (deltaX / 20)), 10)
        updateFrame();
    }

    // Set up and tear down the scroll event listener
    onMount(() => {
      window.onwheel = (event) => {
          handlePan(event);
        event.stopPropagation();
      };
        loadImage();
    });
</script>

<div class="frame-container">
    {#if imageSrc}
        <img draggable={false} src={imageSrc} alt="Video Frame" />
    {:else}
        <p>Loading frame...</p>
    {/if}
</div>

<style>
    .frame-container {
        /* Your styling here */
        overflow: hidden;
        user-select: none;
    }
    img {
        /* Style for the image */
    }
</style>

