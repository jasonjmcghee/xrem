<script>
    export let thumbnail;
    export let appName;
    export let timestamp;
    export let matchingText;
</script>

<div class="thumbnail-card">
    <div class="top-line">
        <div class="app-name">{appName}</div>
        <div class="timestamp">{timestamp}</div>
    </div>
    <img src={thumbnail} alt={`Screenshot of ${appName}`} />
    {#if matchingText}
        <div class="matching-text">{matchingText}</div>
    {/if}
</div>

<style>

    /* Thumbnail Card Styles */
    .thumbnail-card {
        flex: 0 1 30%;
        min-width: 250px;
        height: auto;
        position: relative;
        margin-bottom: 2em;
    }

    .thumbnail-card img {
        width: 100%;
        height: auto;
        object-fit: cover;
        position: absolute;
        top: 1.9em;
    }

    .thumbnail-card .app-name,
    .thumbnail-card .timestamp,
    .thumbnail-card .matching-text {
        position: absolute;
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