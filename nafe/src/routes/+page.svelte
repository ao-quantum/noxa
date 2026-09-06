<script lang="ts">
    import { goto } from '$app/navigation';

    let loginError = $state();

    // on form submit
    async function handleSubmit(event: Event) {
        event.preventDefault(); // get rid of default settings for form submission
        loginError = ''

        const form = event.target as HTMLFormElement;
        const formData = new FormData(form);
        const response = await fetch('/login', { // send login send request to the server
            body: formData,
            method: 'POST',
        });

        const responseBody = await response.json(); // convert the response to json
        if (responseBody.success === true) {
            // Redirect
            goto(responseBody.redirect); // redirect to the page specified by api
        } else {
            loginError = responseBody.message;
        }
    }
</script>

<style>
    /* Set body styles: no spacing, font family, bg colour, and text colour */
    /* Note we have to use :global(body) to apply styles to body */
    /* This is because styles are block scoped, so to apply styles to elements ouside the scope we use :global */
    :global(body) {
        margin: 0 auto;
        padding: 0;
        font-family: Arial, sans-serif;
        background-color: #D16565;
        color: white;
    }

    /* Apply styles for main container to the left of the page: sizing, spacing, width, left float, flexbox setup */
    .container {
        max-width: 40%;
        /* Align it left of the page and make its height use the whole height of the page */
        margin: 0;
        height: 100vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        background-color: rgba(28,28,28,0.32);
        padding-left: 4rem;
        padding-right: 4rem;

    }

    /* this makes the container take up all the page and less padding when screen width is <850px */
    @media (max-width: 850px) {
        .container {
            max-width: 100%;
            padding-left: 1rem;
            padding-right: 1rem;
        }
    }

    /* Add top margin to separate login form from header */
    .main-login {
        margin-top: 2rem;
    }

    /* Align heading text centre of container */
    header {
        text-align: center;
    }

    /* Set font size in heading */
    header h1 {
        font-size: 3rem;
    }

    /* Apply submit button styles */
    /* Setting font size, max width, bg colour, border radius (corner rounding), spacing */
    form button {
        max-width: 20rem;
        font-size: 1.5rem;
        background-color: #832121;
        border-radius: 2px;
        padding: 1rem 4rem;
        margin: 0 auto;
        margin-top: 2rem;
    }

    /* Make the submit button darkred bg colour when mouse hover */
    form button:hover {
        background-color: darkred;
    }

    /* Setup flexbox for form, so we can organise form groups */
    form {
        display: flex;
        flex-direction: column;
    }

    /* Spacing for form groups */
    .form-group {
        margin-bottom: 1rem;
        padding: 1rem 0rem;
    }

    /* Spacing for form labels */
    .form-group label {
        margin-bottom: 0.5rem;
        display: block;
    }

    /* Set spacing, bg colour, and corner rounding for form input fields, and ensure it takes up 100% of the space it could use */
    .form-group input {
        padding: 0.5rem;
        display: block;
        background-color: rgba(74, 74, 74, 0.52);
        border-radius: 5px;
        width: 100%;
    }

</style>

<div class="container">
    <header>
        <h1>Noxa</h1>
    </header>
    <div class="main-login">
        <form method="POST" onsubmit={handleSubmit}>
            <div class="form-group">
                <label for="E-Mail">E-Mail</label>
                <input type="email" id="email" name="email" required>
            </div>
            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" required>
            </div>
            {#if loginError || loginError !== ''}
                <div class="login-error">
                    <p>{loginError}</p>
                </div>
            {/if}
            <button type="submit">Login</button>
        </form>
    </div>
</div>