<script>
    import {onMount} from "svelte";

    export let thumbnail;
    export let frame_number;
    export let timestamp;
    export let matching_text;

    function goToFrameViewerWithFrame(frameNumber) {
        window.location.href = `/timeline#frame${frameNumber}`;
    }

    const loadImage = async () => {
        const binaryData = await (await fetch(`http://localhost:3030/frames/${frame_number}?thumbnail=true`)).arrayBuffer();
        const blob = new Blob([new Uint8Array(binaryData)], { type: 'image/png' });
        thumbnail = URL.createObjectURL(blob);
    };

    onMount(() => {
        loadImage();
    })
</script>

<div class="thumbnail-card" on:click={() => {
    goToFrameViewerWithFrame(frame_number);
}}>
    <div class="top-line">
        <div class="timestamp">{new Date(timestamp).toLocaleString()}</div>
    </div>
    {#if thumbnail}
        <img src={thumbnail} alt={`Screenshot`} />
    {:else}
        <div>Loading...</div>
    {/if}
    {#if matching_text}
        <div class="matching-text">{matching_text}</div>
    {/if}
</div>

<style>

    /* Thumbnail Card Styles */
    .thumbnail-card {
        display: flex;
        flex-direction: column;
        gap: 12px;
        width: 400px;
        height: 300px;
        position: relative;
        margin-bottom: 2em;
        max-width: 90vw;
        justify-content: center;
        align-items: center;
        padding: 16px 40px 32px;
        border-radius: 8px;
    }

    .thumbnail-card:hover {
        background: #ffffff40;
        cursor: pointer;
    }

    .thumbnail-card img {
        width: 100%;
        height: auto;
        object-fit: cover;
        top: 1.9em;
    }

    .thumbnail-card .app-name,
    .thumbnail-card .timestamp,
    .thumbnail-card .matching-text {
        color: #FFFFFF;
    }

    .thumbnail-card .top-line {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .thumbnail-card .app-name {
        left: 0;
        top: 0;
        font-weight: 700;
        font-size: 1rem;
    }

    .thumbnail-card .timestamp {
        right: 0;
        top: 0;
        font-weight: 600;
        font-size: 1rem;
    }

    .thumbnail-card .matching-text {
        left: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        padding: 0.3em;
        border-radius: 0.2em;
    }
</style>