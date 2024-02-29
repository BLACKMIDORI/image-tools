<script>
    import { onMount } from 'svelte';
    onMount(async () => {
        await wasm_bindgen()
    });
</script>
<svelte:head>
    <script src="/wasm/rust_core.js"></script>
</svelte:head>

<slot/>