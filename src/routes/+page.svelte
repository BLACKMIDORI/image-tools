<script>
    import rustCore from "$lib/rust-core"

	/** @type {import('./$types').PageData} */
	// export let data;

	/** @type {string} */
	let inputName = "world";

	function onButtonClick(){
		rustCore.wasm.greet(inputName);
	}
</script>

<h1>Welcome to SvelteKit + Rust(Wasm)</h1>

<input bind:value={inputName}/>
<button on:click={onButtonClick}>Click here to show alert from wasm</button>
