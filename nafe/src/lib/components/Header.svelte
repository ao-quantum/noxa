<script>
    let {title} = $props();
</script>

<style>
    header {
        margin: 1rem;
        padding-bottom: 1rem;
        padding-right: 2rem;
        padding-left: 2rem;
        border-bottom: 2px solid gray;
    }
</style>

<header>
    <h1 class="text-6xl">{title}</h1>
</header>