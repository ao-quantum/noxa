<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_CADRS_BASEURL } from '$env/static/public';
	import Button from '$lib/components/Button.svelte';
	import Header from '$lib/components/Header.svelte';
	import { onMount } from 'svelte';

    const { data } = $props();

    let loaded = $state(false);

    // array of card data
    let cards: {
        id: number;
        front: string;
        back: string;
        last_review: string;
    }[] = $state([]);

    let error: string | null = $state(null);

    async function loadCards() {
        const res = await fetch(`${PUBLIC_CADRS_BASEURL}/cards`, { // fetch all cards under the current user
            method: 'GET',
            headers: [
                ['Authorization', data.sessionId || '']
            ]
        });

        try {
            let json = await res.json(); // parse json from response body
            loaded = true;
    
            if (json.success) {
                cards = json.data; // load cards onto `cards` var
            } else {
                console.error(json.error); // output err
            }
        } catch (e) {
            console.error(e); // output err
            error = 'An error occurred while fetching cards'; // display on page err occured
        }
    }

    onMount(async () => {
        await loadCards(); // load the cards on mount
    })

    async function deleteCard(id: number) { // take id of card to delete
        try {
            await fetch(`${PUBLIC_CADRS_BASEURL}/cards/${id}`, { // send request to delete teh card
                method: 'DELETE',
                headers: [
                    ['Authorization', data.sessionId || '']
                ]
            });

            loadCards(); // reload the cards
        } catch (e) {
            console.error(e);
            error = 'An error occurred while deleting the card';
        }
    }

    export function dateString(date: Date): string {
        // returns the date converted to the format of dd/mm/yyyy
        return `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}`;
    }
</script>

<style>
    table {
        margin: 1rem
    }
    
    td {
        padding: 0.5rem;
    }

    th {
        padding: 0.5rem 0.1rem;
    }
</style>

<Header title="Cards" />

<Button style="success" onclick={() => goto('/app/cards/create')}>Create</Button>

{#if error}
    <p>{error}</p>
{/if}

{#if cards.length > 0 && loaded}
    <table class="table-auto border-collapse border border-gray-600">
        <thead class="table-header-group">
            <tr>
                <th class="border border-gray-600">ID</th>
                <th class="border border-gray-600">Front</th>
                <th class="border border-gray-600">Back</th>
                <th class="border border-gray-600">Last Review</th>
                <th class="border border-gray-600">Delete?</th>
            </tr>
        </thead>
        <tbody>
            {#each cards as card}
                <tr>
                    <td class="border border-gray-600">{card.id}</td>
                    <td class="border border-gray-600">{card.front}</td>
                    <td class="border border-gray-600">{card.back}</td>
                    <td class="border border-gray-600">{card.last_review ? dateString(new Date(card.last_review + 'Z')) : 'Not yet revised'}</td>
                    <td class="border border-gray-600">
                        <Button onclick={() => deleteCard(card.id)} style="error">Delete</Button>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
{:else if loaded}
    <p>No cards found</p>
{:else if loaded === false}
    <p>Loading...</p>
{/if}

