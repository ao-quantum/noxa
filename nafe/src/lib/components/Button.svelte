<script lang="ts">
    let { children, onclick, style } = $props()
</script>

<style>
    button {
        margin: 1rem;
        padding: 0.5rem;
        border-radius: 5px;
        cursor: pointer;
        color: white;
    }
</style>

<button class={style === 'success' ? "bg-green-700" : "bg-red-600"} onclick={onclick}>{@render children()}</button>