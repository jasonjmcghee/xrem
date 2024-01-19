<script>
    import SearchBar from "./SearchBar.svelte";
    import ThumbnailCard from "./ThumbnailCard.svelte";

    let searchTerm = '';
    let frames = [];
    let offset = 0;
    const limit = 3;
    let loading = false;
    let endOfData = false;

    async function fetchFrames() {
        if (loading || endOfData) return;
        loading = true;
        fetch(`http://localhost:3030/frames?limit=${limit}&offset=${offset}&search=${searchTerm}`)
            .then(async response => (await response.json()).data)
            .then(data => {
                frames = [...frames, ...data];
                offset += data.length;
                loading = false;
                if (data.length < limit) {
                    endOfData = true;
                } else {
                    onScroll();
                }
            })
            .catch(error => {
                console.error('Error fetching data:', error);
                loading = false;
            });
    }

    function handleSearch(newTerm) {
        searchTerm = newTerm;
        frames = [];
        offset = 0;
        endOfData = false;
        fetchFrames();
    }

    // Infinite scrolling logic
    function onScroll() {
        if (endOfData) {
            return;
        }
        const scrollY = window.scrollY;
        const visible = document.documentElement.clientHeight;
        const pageHeight = document.documentElement.scrollHeight;
        const bottomOfPage = visible + scrollY >= pageHeight;
        if (bottomOfPage) {
            fetchFrames();
        }
    }

    // Add scroll event listener
    window.addEventListener('scroll', onScroll);

    // Initial fetch
    fetchFrames();
</script>

<SearchBar {searchTerm} onSearch={handleSearch} />

<div class="frames-container">
    {#each frames as frame}
        <ThumbnailCard {...frame} />
    {/each}
</div>
{#if loading}
    <div class="loading-more">Loading more...</div>
{/if}

<style>

    /* Frames Container */
    .frames-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-evenly;
        align-items: flex-start;
        gap: 50px 30px;
        margin: auto;
        padding: 120px 0 0;
        height: auto;
        overflow: auto;
    }

    /* Loading More Indicator */
    .loading-more {
        text-align: center;
        color: #282A2E;
        font-size: 1rem;
    }
</style>