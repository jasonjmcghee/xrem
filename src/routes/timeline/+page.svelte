<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    const minFrameNumber = 1;
    let maxFrameNumber = 1;
    let frameNumber = 1;
    let swipePosition = 1;
    let imageSrc = '';
    let debounceTimer;
    let imageElement;

    function debounce(func, delay) {
        if (imageSrc == "") {
            return function(...args) {
                func.apply(this, args);
            };
        }
        return function(...args) {
            clearTimeout(debounceTimer);
            debounceTimer = setTimeout(() => func.apply(this, args), delay);
        };
    }

    const loadImage = debounce(async () => {
        const binaryData = await (await fetch(`http://localhost:3030/frames/${frameNumber}`)).arrayBuffer();
        const blob = new Blob([new Uint8Array(binaryData)], { type: 'image/png' });
        const src = URL.createObjectURL(blob);
        // if (imageSrc === "") {
            imageSrc = src;
        // }
        // updateImageElement(src);
    }, 100);

    function updateImageElement(src) {
        if (imageElement) {
            // Replace the old image with a new one
            const newImage = document.createElement('img');
            newImage.src = src;
            newImage.alt = 'Video Frame';
            newImage.draggable = false;
            newImage.id = imageSrc;  // This might be unnecessary, consider removing

            // document.querySelector(".frame-container")?.appendChild(newImage);
            setTimeout(() => {
                imageSrc = src;
                // document.querySelector(".frame-container")?.removeChild(newImage);
            }, 0);
        }
    }

    function updateFrame() {
        const newFrame = Math.round(swipePosition);
        if (newFrame !== frameNumber) {
            frameNumber = newFrame;
            loadImage();
        }
    }

    function handlePan(event) {
        const { deltaX } = event;
        swipePosition = Math.min(Math.max(minFrameNumber, swipePosition + (deltaX / 20)), maxFrameNumber)
        updateFrame();
    }

    // Set up and tear down the scroll event listener
    onMount(async () => {
        const maxFrameInfoResponse = await fetch(`http://localhost:3030/frames/max`);
        if (maxFrameInfoResponse.ok) {
            const maxFrameInfo = await maxFrameInfoResponse.json();
            maxFrameNumber = maxFrameInfo.max_frame;
            frameNumber = maxFrameNumber;
            swipePosition = frameNumber;
        }
      window.onwheel = (event) => {
          handlePan(event);
        event.stopPropagation();
      };
        loadImage();
    });
</script>

<div class="frame-container">
    {#if imageSrc}
        <img id={imageSrc} draggable={false} src={imageSrc} alt="Video Frame" />
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

