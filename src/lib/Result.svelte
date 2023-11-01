<!-- Result.svelte -->
<script>
	// import { onMount, createEventDispatcher } from 'svelte';
	import { cubicIn } from 'svelte/easing';
	import { fade } from 'svelte/transition';
	export let uniqueID; // This should be passed as a prop to each Result component
	export let data;
	let audioElement;

	$: {
		if (uniqueID !== data.id && audioElement && !audioElement.paused) {
			audioElement.pause(); // Pause this audio if another one started playing
		}
	}
	function handlePlay() {
		audioElement.play(); // Set the currentlyPlaying store to the ID of the current audio
	}

	function handlePause() {
		if (uniqueID === data.id) {
			audioElement.pause(); // Reset the store if this audio was the one playing
		}
	}
</script>

<!--  -->
<div class="box" in:fade={{ duration: 300, easing: cubicIn }}>
	<p>{data.fileName}</p>
	<p>Integrated Loudness: {data.result} LUFS</p>
	<audio
		bind:this={audioElement}
		on:play={handlePlay}
		on:pause={handlePause}
		src={data.url}
		id="audioPlayer"
		controls
	/>
</div>

<style>
	p {
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	.box {
		border: 2px solid #ccc;
		overflow: auto;
		box-sizing: border-box;
		transition: filter 150ms ease-in-out, transform 0.3s, opacity 0.3s;
		/* max-width: 320px; */
		width: 100%;
		background-color: #0f0f0f;
		color: #fefefe;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.box:hover {
		filter: drop-shadow(0 0 0.5rem #64ffaa65);
	}
	audio {
		padding-bottom: 16px;
	}
</style>
