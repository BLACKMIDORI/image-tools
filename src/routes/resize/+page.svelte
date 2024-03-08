<script lang="ts">
	import rustCore from '$lib/rust-core';
	import debounce from '$lib/debounce';
	import { onMount } from 'svelte';
	onMount(async () => {
		document.addEventListener('wasm_loaded', () => {
			console.log('Setting callback function');
		});
	});
	let dragover = false;

	let images: Array<{ id: number; name: string; width: number; height: number; mimeType: string }> =
		[];
	let imagesState: {
		[id: number]: {
			width: number;
			height: number;
			aspectRatio: undefined | number;
			mimeType: string;
			dataUrl: string;
		};
	} = {};
	let currentState:
		| undefined
		| {
				id: number;
				state: {
					width: number;
					height: number;
					aspectRatio: undefined | number;
					mimeType: string;
					dataUrl: string;
				};
		  } = undefined;

	function handleFiles(files: Array<File>) {
		if (files.length > 0) {
			var file = files[0];
			var reader = new FileReader();
			reader.onload = function (event: any) {
				const dataUrl = event.target.result;
				// rustCore.wasm.
				rustCore.wasm.add_image(file.name, dataUrl);
				images = JSON.parse(rustCore.wasm.get_images_info());
				images.forEach((image) => {
					if (!imagesState[image.id]) {
						currentState = {
							id: image.id,
							state: {
								width: image.width,
								height: image.height,
								aspectRatio: image.width / image.height,
								mimeType: image.mimeType,
								dataUrl: dataUrl
							}
						};
						imagesState[image.id] = currentState!.state;
					}
				});
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

	function scaleImage(width: number, height: number, smooth = false, customFilter = '') {
		const imageId = currentState!.id;
		currentState!.state.width = Math.max(1, Math.round(width));
		currentState!.state.height = Math.max(1, Math.round(height));
		currentState!.state.dataUrl = rustCore.wasm.scale_image(
			imageId,
			currentState!.state.width,
			currentState!.state.height,
			currentState!.state.mimeType,
			smooth,
			customFilter
		);
	}

	function getScaledSize(scale: number, image: { width: number; height: number }) {
		const width = image.width * scale;
		let height = image.height * scale;
		if (currentState?.state.aspectRatio) {
			height = width / currentState?.state.aspectRatio;
		}
		return { width, height };
	}

	let onInputRangeDebounce: Function | undefined;
	function onInputRange(event: any) {
		event.target.value = event.target.value;
		const scale = Number(event.target.value) / 100;
		const imageId = currentState!.id;
		const originalImage = images.find((x) => x.id == imageId);
		const { width, height } = getScaledSize(scale, originalImage!);
		scaleImage(width, height);
		onInputRangeDebounce =
			onInputRangeDebounce ??
			debounce((width: number, height: number) => {
				onInputRangeDebounce = undefined;
				scaleImage(width, height, true);
			}, 300);
		onInputRangeDebounce(width, height);
	}

	function onChangeRange(event: any) {
		const scale = Number(event.target.value) / 100;
		const imageId = currentState!.id;
		const originalImage = images.find((x) => x.id == imageId);
		const { width, height } = getScaledSize(scale, originalImage!);
		scaleImage(width, height, true);
	}
	function onChangeScaleInput(event: any) {
		const scale = Number(event.target.value) / 100;
		const imageId = currentState!.id;
		const originalImage = images.find((x) => x.id == imageId);
		const { width, height } = getScaledSize(scale, originalImage!);
		scaleImage(width, height, true);
	}
	function onChangeWidth(event: any) {
		const width = Number(event.target.value);
		if (currentState?.state.aspectRatio) {
			scaleImage(width, width / currentState!.state.aspectRatio);
		} else {
			scaleImage(width, currentState!.state.height);
		}
	}
	function onChangeHeight(event: any) {
		const height = Number(event.target.value);
		if (currentState?.state.aspectRatio) {
			scaleImage(height * currentState!.state.aspectRatio, height);
		} else {
			scaleImage(currentState!.state.width, height);
		}
	}

	function toggleApectRatio() {
		if (currentState?.state.aspectRatio) {
			currentState!.state.aspectRatio = undefined;
		} else {
			currentState!.state.aspectRatio = currentState!.state.width / currentState!.state.height;
		}
	}

	function calculateScale(imageId: number, width: number, height: number) {
		const originalImage = images.find((x) => x.id == imageId);
		if (!originalImage) {
			return 1;
		}
		return width / originalImage!.width;
	}

	function changeToPng() {
		currentState!.state.mimeType = 'image/png';
		scaleImage(currentState!.state.width, currentState!.state.height, true);
	}
	function changeToJpeg() {
		currentState!.state.mimeType = 'image/jpeg';
		scaleImage(currentState!.state.width, currentState!.state.height, true);
	}
	function changeToWebP() {
		currentState!.state.mimeType = 'image/webp';
		scaleImage(currentState!.state.width, currentState!.state.height, true);
	}
	function renderCustom(customFilter: string) {
		switch (customFilter) {
			case 'Nearest':
				scaleImage(currentState!.state.width, currentState!.state.height, false);
				break;
			case 'Triangle':
				scaleImage(currentState!.state.width, currentState!.state.height, false, 'Triangle');
				break;
			case 'CatmullRom':
				scaleImage(currentState!.state.width, currentState!.state.height, false, 'CatmullRom');
				break;
			case 'Lanczos3':
				scaleImage(currentState!.state.width, currentState!.state.height, false, 'Lanczos3');
				break;
		}
	}

	function replaceExtensionWithCurrentOne(filename: string) {
		let extension: string;
		switch (currentState?.state.mimeType) {
			case 'image/png':
				extension = 'png';
				break;
			case 'image/jpeg':
				extension = 'jpeg';
				break;
			case 'image/webp':
				extension = 'webp';
				break;
			default:
				extension = 'bin';
				break;
		}
		const parts = filename.split('.');
		parts.pop();
		parts.push(extension);
		return parts.join('.');
	}
	var gcd = function (a: number, b: number) {
		if (!b) {
			return a;
		}

		return gcd(b, a % b);
	};
</script>

<div class="flex flex-row h-[calc(100%-130px)] w-[100dvw]">
	<div class="flex-1 pr-2 border-r-2 border-[theme(colors.grey)]">
		<div class="h-[100%] flex flex-col w-[100%]">
			<div
				aria-hidden="true"
				class="{dragover
					? 'border-gray-200'
					: 'border-[rgba(0,0,0,0)] border-b-[theme(colors.grey)]'} w-[100%] flex flex-row border-2 border-dashed h-[100px]"
				on:dragleave={(event) => (dragover = false)}
				on:dragover={(event) => {
					event.preventDefault();
					dragover = true;
				}}
				on:drop={(event) => {
					dragover = false;
					handleDrop(event);
				}}
			>
				<div class="whitespace-nowrap overflow-auto flex">
					{#each Object.entries(imagesState) as [key, imageState]}
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
						<img
							on:click={() => (currentState = { id: Number(key), state: imageState })}
							alt="image id {key}"
							src={imageState.dataUrl}
							class="inline-block border-2 {currentState?.id == Number(key)
								? 'border-[theme(colors.MIDORI)]'
								: 'border-[rgba(0,0,0,0)]'}"
						/>
					{/each}
				</div>
				<div class="flex-1" />
				<div
					class="{!dragover
						? 'border-[theme(colors.grey)]'
						: 'border-[rgba(0,0,0,0)]'} ml-2 border-2 border-dashed rounded-2xl min-w-[120px] h-[90px] text-center"
				>
					<input class="hidden" type="file" id="fileInput" on:change={onFilesChange} />
					<label class="block w-[100%] h-[100%] flex items-center justify-center" for="fileInput">
						<div class="">
							<p>Drag & Drop</p>
							<p>or</p>
							<p>Browse</p>
						</div>
					</label>
				</div>
			</div>
			<div class="flex-1">
				{#if currentState !== undefined}
					<div class="h-[100%] flex flex-col justify-center">
						<div class="relative flex justify-center">
							<div class="absolute right-0 top-2">
								<button
									on:click={changeToPng}
									class="{currentState.state.mimeType != 'image/png'
										? 'text-[#777]'
										: 'text-[#ccc]'} border-2 rounded-2xl p-1">PNG</button
								>
								<button
									on:click={changeToJpeg}
									class="{currentState.state.mimeType != 'image/jpeg'
										? 'text-[#777]'
										: 'text-[#ccc]'} border-2 rounded-2xl p-1">JPEG</button
								>
								<button
									on:click={changeToWebP}
									class="{currentState.state.mimeType != 'image/webp'
										? 'text-[#777]'
										: 'text-[#ccc]'} border-2 rounded-2xl p-1">WEBP</button
								>
								<a
									href={currentState.state.dataUrl}
									download={replaceExtensionWithCurrentOne(images[currentState.id].name)}
								>
									<button class="p-1 rounded-2xl border-2"> Download </button>
								</a>
							</div>
							<span> Size: {currentState.state.width}x{currentState.state.height} </span>
							<span>&nbsp;</span>
							<span
								>Aspect ratio: {currentState.state.width /
									gcd(currentState.state.width, currentState.state.height)}:{currentState.state
									.height / gcd(currentState.state.width, currentState.state.height)}</span
							>
						</div>
						<div class="flex-1">
							<div class="h-[100%] flex align-center justify-center">
								<img
									class="max-h-96 self-center"
									id="preview"
									src={currentState.state.dataUrl}
									alt="Preview"
								/>
							</div>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>
	<div class="p-2 overflow-auto">
		<div>
			<input
				disabled={!currentState}
				type="range"
				min="0"
				max="200"
				value={currentState
					? calculateScale(currentState.id, currentState.state.width, currentState.state.height) *
						100
					: 100}
				class="slider"
				on:input={onInputRange}
				on:change={onChangeRange}
			/>
			<br />
			<br />
			Scale (%):
			<input
				disabled={!currentState}
				class="w-[50px]"
				type="number"
				value={(currentState
					? calculateScale(currentState.id, currentState.state.width, currentState.state.height) *
						100
					: 100
				).toFixed()}
				on:input={onChangeScaleInput}
			/>
			<br />
			<br />
			<span>
				Size: <input
					disabled={!currentState}
					class="w-[50px]"
					type="number"
					step="1"
					value={currentState?.state.width ?? 0}
					on:input={onChangeWidth}
				/>
				x
				<input
					disabled={!currentState}
					class="w-[50px]"
					type="number"
					step="1"
					value={currentState?.state.height ?? 0}
					on:input={onChangeHeight}
				/>
				&nbsp;&nbsp;
			</span>

			<br />
			<br />
			<input
				type="checkbox"
				disabled={!currentState}
				checked={currentState ? !!currentState.state.aspectRatio : true}
				on:change={toggleApectRatio}
			/>
			Lock aspect ratio
			<br />
			<br />
			{#if currentState?.state.mimeType == 'image/webp'}
				WebP doesn't render?
				<br />
				Try rendering once:
				<br />
				<button on:click={() => renderCustom('Nearest')}>Nearest(fast)</button>
				<br />
				<button on:click={() => renderCustom('CatmullRom')}>CatmullRom(smooth)</button>
				<br />
				<button on:click={() => renderCustom('Triangle')}>Triangle</button>
				<br />
				<button on:click={() => renderCustom('Lanczos3')}>Lanczos3</button>
			{/if}
		</div>
	</div>
</div>

<style>
	:global(body) {
		height: 100dvh;
	}
</style>
