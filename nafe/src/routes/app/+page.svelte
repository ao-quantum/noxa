<script lang="ts">
	import { PUBLIC_CADRS_BASEURL } from '$env/static/public';
	import { onMount } from 'svelte';
	import Header from '$lib/components/Header.svelte';

    let { data } = $props();

    let loaded = $state(false);

    let stats: {
        total: number;
        due: number;
        not_reviewed_yet: number;
        stability_mean: number;
    } | null = $state(null);

    let error: string | null = $state(null);

    onMount(async () => {
        const res = await fetch(`${PUBLIC_CADRS_BASEURL}/cards/stats`, { // fetch stats from API
            method: 'GET',
            headers: [
                ['Authorization', data.sessionId || '']
            ]
        });

        try {
            let json = await res.json(); // parse data
            loaded = true;
    
            if (json.success) {
                stats = json.data; // assign the data to the stats variable
            } else {
                console.error(json.error);
            }
        } catch (e) {
            console.error(e);
            error = 'An error occurred while fetching cards';
        }
    });
</script>

<style>
    .stats {
        margin: 1rem;
    }

    .stat {
        text-align: center;
        background-color: #4CAF50;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        height: 100%;
        padding: 4rem 1rem;
    }

    .stat-title {
        font-size: 1.5rem;
        color: white;
        float: left;
        width: 50%;
    }

    .stat-value {
        font-size: 2rem;
        color: white;
        float: right;
        width: 50%;
    }

</style>

<Header title="Dashboard" />

{#if error}
    <p>{error}</p>
{/if}

{#if stats !== null && loaded}
    <div class="stats grid grid-cols-2 gap-4">
        <div class="stat">
            <div class="stat-title">Total Cards</div>
            <div class="stat-value">{stats.total}</div>
        </div>
        <div class="stat">
            <div class="stat-title">Due cards</div>
            <div class="stat-value">{stats.due}</div>
        </div>
        <div class="stat">
            <div class="stat-title">Not Reviewed Yet</div>
            <div class="stat-value">{stats.not_reviewed_yet}</div>
        </div>
        <div class="stat">
            <div class="stat-title">Mean Stability</div>
            <div class="stat-value">{stats.stability_mean ?? 'N/A'}</div>
        </div>
    </div>
{:else if loaded === true}
    <p>No cards found</p>
{:else if loaded === false}
    <p>Loading...</p>
{/if}
