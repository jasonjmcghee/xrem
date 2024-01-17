<script>
    export let searchTerm;
    export let onSearch;

    function handleInput(event) {
        onSearch(event.target.value);
    }
</script>

<div class="search-bar-container">
<input type="text" bind:value={searchTerm} on:input={handleInput} placeholder="Search for anything..." class="search-bar" />
</div>

<style>
    /* Search Bar Styles */
    .search-bar-container {
        display: flex;
        justify-content: center;
        align-items: center;
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        padding-top: 1.2em;
        padding-bottom: 1.2em;
        width: 100%;
        margin: auto;
        z-index: 100;
        background: linear-gradient(0deg, #1e1e1e00 0%, #1e1e1ee8 75%);
        max-width: 90vw;
    }

    .search-bar {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 0.8em 1.2em;
        background: #282A2E;
        border-radius: 0.5em;
        width: 100%;
        color: #B2B2B2;
        font-size: 1.25rem;
    }
</style>