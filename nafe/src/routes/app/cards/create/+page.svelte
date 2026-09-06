<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_CADRS_BASEURL } from '$env/static/public';
	import Header from '$lib/components/Header.svelte';

    let { data } = $props();

    let error: string | null = $state(null);

    // form submit handler
    async function submitCreate(event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement}) {
        error = ''; // reset error message
        event.preventDefault(); // prevent default form submission
        const form = event.currentTarget;
        const formData = new FormData(form); // parse FormData from form

        const front = formData.get('front');
        const back = formData.get('back');

        if (!front || !back || typeof front !== 'string' || typeof back !== 'string') { // check if the front or back field is empty
            error = 'Front or back field is empty';
            return;
        }

        if (front.length > 255 || back.length > 65535) { // check if the front or back field is too long
            error = 'Front or back field is too long';
            return;
        }

        try {
            await fetch(`${PUBLIC_CADRS_BASEURL}/cards`, { // send request to create a new card
                method: 'POST',
                headers: [
                    ['Authorization', `${data.sessionId}`],
                ],
                body: JSON.stringify({
                    front, // front field of form data
                    back, // back field of form data
                })
            });

            goto('/app/cards'); // redirect to cards page after creation
        } catch (e) {
            console.error(e);
            error = 'An error occurred while creating the card';
        }

    }

</script>

<style>
    /* Setup flexbox and margin */
    form {
        display: flex;
        flex-direction: column;
        margin: 1rem;
    }

    /* For the form field labels. Set margins */
    label {
        margin: 0.5rem;
        margin-left: 1rem;
    }

    /* Textarea form inputs. Set the style and spacing */
    textarea {
        margin: 1rem;
        padding: 0.5rem;
        resize: none;
        background-color: rgb(68, 70, 81);
        border-radius: 5px;
    }

    /* For the submit button. Set the sizing, spacing and style  */
    button {
        margin: 1rem;
        padding: 0.5rem;
        color: white;
        border: none;
        border-radius: 5px;
    }
</style>

<Header title="Create a new card" />

{#if error}
    <p>{error}</p>
{/if}

<form onsubmit={submitCreate}>
    <label for="front" class="text-2xl">Front</label>
    <textarea id="front" name="front" required></textarea>
    <label for="back" class="text-2xl">Back</label>
    <textarea id="back" name="back" required></textarea>
    <button type="submit" class="bg-green-700">Create</button>
</form>
