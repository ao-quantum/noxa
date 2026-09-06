<script lang="ts">

    const { data, children } = $props();

</script>

<style>
    /* Set main container to take up whole page & bg colour */
    .main {
        margin: 0 auto;
        width: 100%;
        color: rgb(211, 211, 211);
    }

    /* Set left side bar area, left float, bg colour, and height */
    .sidebar {
        width: 20%;
        float: left;
        background-color: #2B2727;
        min-height: 100vh;
        position: fixed;
    }

    /* Top of the sidebar with username and app name */
    /* Set the spacing, center text and bottom border/line */
    .sidebar .top {
        text-align: center;
        padding: 2rem;
        border-bottom: 1px solid rgb(146, 146, 146);
        margin: 0 2rem;
    }

    /* Spacing for the p element showing the username */
    .sidebar .top p {
        margin-top: 1rem;
    }

    /* Navbar spacing/margin and text align */
    .sidebar .nav {
        margin: 2rem;
        text-align: center;
    }

    /* Navbar list settings & spacing */
    .sidebar .nav ul {
        list-style-type: none; /* Remove bullet points */
        padding: 0;
    }
    .sidebar .nav ul li {
        margin: 0.5rem 0; /* Top and bottom of every navbar button to have 0.5rem margin, no side margin*/
        padding: 1rem; /* 1rem padding for inside the button */
    }
    .sidebar .nav ul li:last-child { /* If its the last list item/navbar button */
        margin: 1rem 0; /* Set top+bottom margin to 1rem */
    }
    .sidebar .nav ul li:hover { /* When any navbar button has mouse hover */
        background-color: rgb(199, 32, 32, 0.2); /* Change bg colour */
    }

    /* Styles for actual content area/whatever the page file specifies, to the right majority of the page */
    .content {
        width: 80%;
        float: right;
        min-height: 100vh;
        background-color: #6F3C3C;
    }
</style>

<div class="main">
    <div class="sidebar">
        <div class="top">
            <h1 class="text-4xl">Noxa</h1>
            <p>{data.username}</p>
        </div>
        <div class="nav">
            <ul>
                <a href="/app">
                    <li>Home</li>
                </a>
                <a href="/app/cards">
                    <li>Cards</li>
                </a>
                <a href="/app/practice">
                    <li>Practice</li>
                </a>
                <a href="/logout">
                    <li>Log out</li>
                </a>
            </ul>
        </div>
    </div>
    <div class="content">
        {@render children()}
    </div>
</div>