<script>
	import '../app.css';
	import { onMount } from 'svelte';
	onMount(async () => {
        if(window.location.pathname!="/resize"){
            window.location.href = "/resize"
        }
		await wasm_bindgen();
        document.dispatchEvent(new Event("wasm_loaded"))
        wasm_bindgen.initialize();
	});

	const menuItems = [{ title: 'Resize', href: '/resize' }];
</script>

<svelte:head>
	<script src="/wasm/rust_core.js"></script>
</svelte:head>

<nav class="p-4 border-b-[1px] border-[theme(colors.grey)]">
	<div class="relative left-0 top-0">
		<span class="absolute text-2xl"> Image Tools </span>
		<span class="absolute right-0 text-2xl">
			<span  class="text-[theme(colors.BLACK)] bg-[theme(colors.MIDORI)]">BLACK</span><span class="text-[theme(colors.MIDORI)]">MIDORI</span>
		</span>
	</div>
	<ul class="flex justify-center h-[2rem]">
		{#each menuItems as menuItem}
			<li class="inline"><a href={menuItem.href}>{menuItem.title}</a></li>
		{/each}
	</ul>
</nav>
<slot />
<footer class="text-center p-[20px] border-t-[1px] border-[theme(colors.grey)]">
    <i class="fa fa-github"></i>
    GitHub 👉 <a href="https://github.com/BLACKMIDORI/image-tools">BLACKMIDORI/image-tools</a>
</footer>
<style lang="postcss">
	:global(html) {
		background-color: theme(colors.BLACK);
		color: theme(colors.white);
	}
	:global(button) {
		color: theme(colors.MIDORI);
	}
	:global(input) {
		background-color: #000000;
		border: 1px solid theme(colors.MIDORI);
		border-radius: 7px;
	}
	:global(input:focus) {
		background-color: theme(colors.darkpurple);
	}
	:global(a) {
		color: theme(colors.purple);
	}
	:global(a:hover) {
		text-decoration: underline;
    }
</style>
