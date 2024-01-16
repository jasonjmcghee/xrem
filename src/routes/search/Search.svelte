<script>
    export let searchTerm;
    export let onSearch;

    function handleInput(event) {
        onSearch(event.target.value);
    }
</script>

<input type="text" bind:value={searchTerm} on:input={handleInput} placeholder="Search for anything..." class="search-bar" />

<style>
    /* Search Bar Styles */
    .search-bar {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 1.2em 1.4em;
        background: #282A2E;
        border-radius: 0.5em;
        width: 100%;
        max-width: 90vw;
        color: #B2B2B2;
        font-size: 1.25rem;
    }
</style>