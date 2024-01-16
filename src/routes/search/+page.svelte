<script>
    import SearchBar from "./SearchBar.svelte";
    import ThumbnailCard from "./ThumbnailCard.svelte";

    let searchTerm = '';
    let frames = [];
    let offset = 0;
    const limit = 20;
    let loading = false;
    let endOfData = false;

    async function fetchFrames() {
        if (loading || endOfData) return;
        loading = true;
        fetch(`http://localhost:3030/frames?limit=${limit}&offset=${offset}`)
            .then(response => response.json())
            .then(data => {
                if (data.length < limit) endOfData = true;
                frames = [...frames, ...data];
                offset += data.length;
                loading = false;
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
    <div>Loading more...</div>
{/if}

<style>

    /* Frames Container */
    .frames-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-between;
        align-items: flex-start;
        gap: 10vw 4.5vw;
        padding: 0;
        margin: 3.5vh auto;
        width: 95%;
        max-width: 90vw;
    }

    /* Loading More Indicator */
    .loading-more {
        text-align: center;
        color: #282A2E;
        font-size: 1rem;
    }

    /* Responsive Design */
    @media (max-width: 768px) {
        .frames-container {
            gap: 5vw 2.5vw;
        }

        .thumbnail-card {
            flex-basis: 45%;
        }

        .search-bar {
            padding: 1em;
            font-size: 1rem;
        }
    }

    @media (max-width: 480px) {
        .frames-container {
            flex-direction: column;
            align-items: center;
            gap: 3vh 0;
        }

        .thumbnail-card {
            flex-basis: 90%;
        }
    }
</style>