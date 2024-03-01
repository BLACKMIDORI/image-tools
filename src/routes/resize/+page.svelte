<script lang="ts">
	import rustCore from '$lib/rust-core';
	import { onMount } from 'svelte';
	onMount(async () => {
		document.addEventListener('wasm_loaded', () => {
			console.log('Setting callback function');
		});
	});

	let imageSrc: string | null = null;

	let width = 16;
	let height = 9;
	let scale = 1;
    let step = 1;

	function handleFiles(files: Array<File>) {
		if (files.length > 0) {
			var file = files[0];
			var reader = new FileReader();
			reader.onload = function (event: any) {
				const dataUrl = event.target.result;
				rustCore.wasm.set_image(dataUrl);
				imageSrc = rustCore.wasm.scale_image(1, width/height);
			};
			reader.readAsDataURL(file);
		}
	}
	function onFilesChange(event: any) {
		const files = event.target.files;
		handleFiles(files);
	}

	function handleDrop(event: any) {
		event.preventDefault();
		var dt = event.dataTransfer;
		var files = dt.files;

		handleFiles(files);
	}

	function onChangeSlider(event: any) {
		scale = Number(event.target.value) / 100;
		imageSrc = rustCore.wasm.scale_image(scale, width/height);
	}
    function onChangeSliderInput(event:any){
		scale = Number(event.target.value) / 100;
		imageSrc = rustCore.wasm.scale_image(scale, width/height);
    }
    function onChangeStepInput(event:any){
		step = Number(event.target.value) ;
    }
    function onChangeWidth(event:any){
        width = Number(event.target.value)
		imageSrc = rustCore.wasm.scale_image(scale, width/height);
    }
    function onChangeHeight(event:any){
        height = Number(event.target.value)
		imageSrc = rustCore.wasm.scale_image(scale, width/height);
    }

</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="drop-area"
	id="drop-area"
	on:dragover={(event) => event.preventDefault()}
	on:drop={handleDrop}
>
	<p>Drag & Drop your image here</p>
	<p>or</p>
	<input class="hidden" type="file" id="fileInput" on:change={onFilesChange} />
	<label for="fileInput">Browse</label>
</div>
{#if imageSrc !== null}
	<div class="w-full">
		<input type="range" min="0" max="200" value="100" class="slider" on:input={onChangeSlider} />
        <input type="number" step={step} value={scale*100} on:input={onChangeSliderInput}/>

        Step:
        <input type="number" step={0.1} value={step} on:input={onChangeStepInput}/>
	</div>
    <br/>
	<span> Aspect ratio: <input type="number" step="1" value={width} on:change={onChangeWidth}>:<input type="number" step="1" value={height} on:change={onChangeHeight}></span>
    <br/>
	<span> Size: {(scale*(width/height) * 500).toFixed()}x{scale * 500} </span>
	<img id="preview" src={imageSrc} alt="Preview" />
{/if}

<style>
	.drop-area {
		border: 2px dashed #ccc;
		border-radius: 20px;
		padding: 20px;
		text-align: center;
		font-family: Arial, sans-serif;
		margin: 20px auto;
		width: 300px;
	}
	.drop-area.highlight {
		border-color: #007bff;
	}
</style>
