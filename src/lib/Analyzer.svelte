<!-- Analyzer.svelte -->
<script>
	export let worker;
	import { onMount, createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { process } from './store.js';
	import { backInOut } from 'svelte/easing';
	let dragover = true;
	let dropzone;
	let audioFileInput;
	let startTime;
	let fileName = '';
	let url = '';
	const dispatch = createEventDispatcher();
	async function extractAudioInfo(file, objectURL) {
		// const asset = AV.Asset.fromFile(file);
		// let format = await new Promise((resolve) => asset.get("format", resolve));
		// const sampleRate = format.sampleRate;
		// const formatID = format.formatID;
		// const numberOfChannels = format.channelsPerFrame;
		// if (numberOfChannels > 2) {
		//   throw new Error(
		//     "Only mono or stereo audio files are supported currently for Integrated Loudness measurement."
		//   );
		// }
		// const bitrate = format.bitsPerChannel;
		// console.log(file.type, formatID, sampleRate, numberOfChannels, bitrate);
		try {
			const tmpContext = new (window.AudioContext || window.webkitAudioContext)();
			const response = await fetch(objectURL);
			const buffer = await response.arrayBuffer();
			let decoded;
			const data = await new Promise((resolve, reject) => {
				tmpContext.decodeAudioData(
					buffer,
					(decodedBuffer) => {
						decoded = decodedBuffer;
						resolve({
							sampleLength: decodedBuffer.length,
							sampleRate: decodedBuffer.sampleRate,
							numberOfChannels: decodedBuffer.numberOfChannels
						});
					},
					reject
				);
			});
			//   const audioContext = new OfflineAudioContext(
			//     data.numberOfChannels,
			//     data.sampleLength,
			//     data.sampleRate
			//   );
			//   console.log(audioContext);

			//   const decodedSamplesBuffer = await audioContext.decodeAudioData(buffer);
			return new AudioData(decoded, data.sampleRate);
		} catch (error) {
			process.set(false);
			console.error('Decoding error:', error);
		}
	}

	async function extractAudioData(audioBuffer) {
		let channelsData = [];
		for (let channel = 0; channel < audioBuffer.numberOfChannels; channel++) {
			let channelData = audioBuffer.getChannelData(channel);
			let audioDataArray = new Float32Array(channelData.length);
			for (let i = 0; i < channelData.length; i++) {
				audioDataArray[i] = channelData[i];
			}
			channelsData.push(audioDataArray);
		}

		return channelsData;
	}

	class AudioData {
		constructor(samplesBuf, sampleRate) {
			(this.samplesBuf = samplesBuf), (this.sampleRate = sampleRate);
		}
	}

	async function handleDrop(event) {
		event.preventDefault();
		dragover = false;
		// // Ensure there's at least one file dropped
		if (event.dataTransfer.items && event.dataTransfer.items.length > 0) {
			const file = event.dataTransfer.items[0].getAsFile();
			if (file.type !== 'audio/wav') {
				alert('Please upload a .wav file');
				event.target.value = '';
				throw new Error('Please upload a .wav file');
			} else {
				process.set(true);
				fileName = file.name;
				console.log('fileName after file drop:', fileName);
				const objectURL = URL.createObjectURL(file);
				try {
					const audioData = await extractAudioInfo(file, objectURL);
					const channelsData = await extractAudioData(audioData.samplesBuf);
					startTime = Date.now();
					worker.postMessage({
						action: 'processAudio',
						buffer: channelsData,
						sampleRate: audioData.sampleRate
					});
					url = objectURL;
				} catch (error) {
					alert(error.message);
				}
			}
		}
	}

	function handleDragEnter(event) {
		event.preventDefault();
	}

	function handleDragOver(event) {
		event.preventDefault();
		dragover = true;
	}

	async function handleFileUpload(event) {
		const file = event.target.files[0];
		if (file) {
			if (file.type !== 'audio/wav') {
				alert('Please upload a .wav file');
				event.target.value = '';
				throw new Error('Please upload a .wav file');
			} else {
				process.set(true);
				fileName = file.name;
				console.log('fileName after file upload:', fileName);
				const objectURL = URL.createObjectURL(file);
				try {
					const audioData = await extractAudioInfo(file, objectURL);
					const channelsData = await extractAudioData(audioData.samplesBuf);
					startTime = Date.now();
					worker.postMessage({
						action: 'processAudio',
						buffer: channelsData,
						sampleRate: audioData.sampleRate
					});
					url = objectURL;
				} catch (error) {
					alert(error.message);
				}
			}
		}
	}

	onMount(async () => {
		startTime = Date.now();
		worker.onmessage = (event) => {
			switch (event.data.type) {
				case 'initialized':
					console.log('WebWorker and WASM initialized');
					break;
				case 'error':
					process.set(false);
					console.error('WebWorker error:', event.data.error);
					break;
				case 'loudnessResult':
					dispatch('analyzerFinished', {
						fileName: fileName,
						result: event.data.value,
						url: url
					});
					process.set(false);
					const processingTime = Date.now() - startTime;
					console.log(`Processing time: ${processingTime} ms`);
					console.log('Integrated Loudness:', event.data.value);
					break;
				default:
					console.log('Received from WebWorker:', event.data);
			}
		};
	});
</script>

{#if $process}
	<div
		class="processing-screen"
		in:fade={{ duration: 300, easing: backInOut }}
		out:fade={{ duration: 200, easing: backInOut }}
	>
		<p>Processing audio, please wait...</p>
	</div>
{:else}
	<div
		bind:this={dropzone}
		role="button"
		class="dropzone"
		tabindex="0"
		on:drop={handleDrop}
		on:dragover={handleDragOver}
		on:dragenter={handleDragEnter}
		on:dragover={handleDragOver}
		aria-label="Upload audio by dropping here"
	>
		Drop your audio file here or
		<input
			type="file"
			id="fileInput"
			class="hidden"
			bind:this={audioFileInput}
			on:change={handleFileUpload}
			accept="audio/wav"
		/>
		<label for="fileInput" class="custom-file-upload">Choose File</label>
		<span id="fileName">No file chosen</span>
	</div>
{/if}

<style>
	.hidden {
		display: none;
	}

	.custom-file-upload {
		padding: 10px 15px;
		border: 1px solid #ccc;
		display: inline-block;
		cursor: pointer;
		border-radius: 10px;
		will-change: filter;
		transition: filter background-color 150ms ease-in-out;
	}
	.custom-file-upload:hover {
		background-color: #a4f6b786;
		filter: drop-shadow(0 0 1rem #64ffaa65);
	}

	p {
		color: #242424;
		font-size: xx-large;
	}

	#fileName {
		margin-left: 10px;
		font-style: italic;
	}
	.dropzone {
		padding: 20px;
		text-align: center;
		cursor: pointer;
		color: #fefefe;
		font-size: large;
		transition: background-color 0.2s ease-in-out, color 0.2s linear, transform 0.2s ease-in-out,
			border 0.2s ease-in-out;
	}

	.dropzone:hover {
		background-color: #242424;
		transform: scale(1.25);
	}
	.processing-screen {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: rgba(255, 255, 255, 0.497);
		z-index: 1000;
		transition: background-color 0.2s ease-in-out;
		text-align: center;
	}
</style>
