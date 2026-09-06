<script lang="ts">
	import { PUBLIC_CADRS_BASEURL } from '$env/static/public';
	import { onMount } from 'svelte';
	import type { PageProps } from '../$types';
	import Header from '$lib/components/Header.svelte';

    let { data }: PageProps = $props();

    let flipped = $state(false);
    let error: string | null = $state(null);

    function flip() {
        flipped = !flipped;
    }

    // data of the current card being reviewed
    let currentCard: {
        id: number;
        front: string;
        back: string;
    } | null = $state(null)

    async function nextCard() {
        let res = await fetch(`${PUBLIC_CADRS_BASEURL}/cards/next`, { // fetch next card from API
            method: 'GET',
            headers: [
                ['Authorization', data.sessionId || '']
            ]
        });
        let json = await res.json(); // parse json

        if (!json.data) { // Do not continue if no card
            return;
        }

        currentCard = { // assign the data to the currentCard variable
            id: json.data.id,
            front: json.data.front,
            back: json.data.back,
        };
    }

    async function grade(grade: number) {
        let res = await fetch(`${PUBLIC_CADRS_BASEURL}/cards/review`, { // send card rating to API
            method: 'PATCH',
            headers: [
                ['Authorization', data.sessionId || ''],
                ['Content-Type', 'application/json']
            ],
            body: JSON.stringify({
                card_id: currentCard?.id, // card id reviewed
                grade // grade achieved
            })
        });

        try {
            let json = await res.json(); // parse json
    
            if (json.success) {
                await nextCard(); // fetch next card
                flip(); // hide card
            } else {
                console.error(json.error); // log err
                error = json.error; // display error on page
            }
        } catch (e) {
            console.error(e);
            error = 'An error occurred while grading the card'; // display on page err occured
        }
    }

    onMount(() => {
        nextCard(); // load card on mount
    })
</script>

<style>
    .card-container {
        text-align: center; /* align text centre on centre of card */
    }
    /* set spacing, alignment, sizing and colour of card box */
    .card {
        background-color: rgb(124, 86, 38);
        max-width: 40%;
        text-align: center;
        padding: 3rem;
        margin: 0 auto;
    }
    .card:hover {
        cursor: pointer; /* change cursor to pointer on hover */
    }

    /* setup ratings flexbox for rating buttons and set margin  */
    .ratings {
        display: flex;
        justify-content: space-around;
        margin-top: 2rem;
    }
    .ratings button {
        padding: 2rem 6rem; /* set padding */
        user-select: none; /* prevent text selection */
    }
    .ratings button:hover {
        cursor: pointer; /* change cursor to pointer on hover */
    }
</style>

<Header title="Practice" />

{#if error}
    <h3>{error}</h3>
{/if}

{#if !currentCard}
    <p>No cards to practice</p>
{:else}
    <div class="card-container">
        <h4 style="min-height: 2rem;">
            {#if flipped}{currentCard.front}{/if}
        </h4>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="card" onclick="{flip}">
            <div class="focus">
                {#if flipped}
                    {currentCard.back}
                {:else}
                    {currentCard.front}
                {/if}
            </div>
        </div>
    </div>
    <div class="ratings" style={`display: ${flipped ? 'flex' : 'none'}`}>
        <button class="bg-red-900" onclick={() => grade(1)}>Forgot</button>
        <button class="bg-amber-700" onclick={() => grade(2)}>Hard</button>
        <button class="bg-green-700" onclick={() => grade(3)}>Good</button>
        <button class="bg-green-900" onclick={() => grade(4)}>Easy</button>
    </div>
{/if}