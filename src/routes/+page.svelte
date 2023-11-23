<!-- App.svelte -->
<script>
	import { onDestroy, onMount, tick } from 'svelte';
	import Result from '../lib/Result.svelte';
	import { flip } from 'svelte/animate';
	import NavBar from '../lib/NavBar.svelte';
	import Analyzer from '../lib/Analyzer.svelte';
	import { loaded, results } from '../lib/store.js';
	import { quadInOut } from 'svelte/easing';
	import ExpandLess from '../expand_less_FILL0_wght400_GRAD0_opsz24.svg';
	import ExpandMore from '../expand_more_FILL0_wght400_GRAD0_opsz24.svg';

	let worker;
	let audioPlayer;

	let draggedItemIndex = null;
	let overIndex = null;

	function handleDragStart(event, index) {
		draggedItemIndex = index;
		event.target.style.opacity = '0.5';
	}

	function handleDragOver(event) {
		event.preventDefault();
	}

	function handleDrop(event, index) {
		event.preventDefault();
		if (draggedItemIndex !== null) {
			const updatedResults = [...$results];
			[updatedResults[draggedItemIndex], updatedResults[index]] = [
				updatedResults[index],
				updatedResults[draggedItemIndex]
			];
			results.set(updatedResults);
			event.target.style.opacity = '1';
		}
	}

	function handleDragEnd(event) {
		event.target.style.opacity = '1';
	}

	function handleDragEnter(event, index) {
		overIndex = index;
	}

	function handleDragLeave(event) {
		overIndex = null;
	}

	function handleFileNameUpdate(event) {
		const { fileName, result, url } = event.detail;
		let currentResults = $results;
		let newResults = [
			...currentResults,
			{
				id: currentResults.length,
				fileName,
				result,
				url,
				isProcessed: true
			}
		];
		results.set(newResults);
		console.log($results);
		handleScrollToBottom();
	}

	function handleScrollToTop() {
		const container = document.querySelector('.analyzers-container');
		container.scrollTop = 0;
	}

	async function handleScrollToBottom() {
		await tick();
		const scrollHeight = document.body.scrollHeight;
		const visibleHeight = window.innerHeight;
		const scrollableHeight = scrollHeight - visibleHeight;
		const container = document.querySelector('.analyzers-container');
		if (container.scrollHeight > container.clientHeight) {
			// Check for overflow
			container.scrollTop = container.scrollHeight;
		}
		window.scrollTo({
			top: scrollableHeight,
			behavior: 'smooth'
		});
	}

	onMount(async () => {
		worker = new Worker('/audioProcessor.worker.js');
		loaded.set(true);
		audioPlayer = document.getElementById('audioPlayer');
	});

	onDestroy(() => {
		worker?.terminate();
	});
</script>

<NavBar />
{#if $loaded}
	<div class="analyzer">
		<Analyzer {worker} on:analyzerFinished={handleFileNameUpdate} />
	</div>
	<div class="controller">
		<div class="analyzers-container">
			{#each $results.filter((item) => item.isProcessed) as resultData, index (resultData.id)}
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div
					class="grid"
					draggable="true"
					on:dragstart={(event) => handleDragStart(event, index)}
					on:dragover={handleDragOver}
					on:dragenter={(event) => handleDragEnter(event, index)}
					on:dragleave={handleDragLeave}
					on:drop={(event) => handleDrop(event, index)}
					on:dragend={handleDragEnd}
					animate:flip={{ duration: 420, easing: quadInOut }}
				>
					<Result data={resultData} uniqueID={index} />
				</div>
			{/each}
		</div>
		<div class="controller-btns">
			<button id="scrollTop" on:click={handleScrollToTop}
				><img src={ExpandLess} alt="Expand Less" /></button
			>
			<button id="scrollBottom" on:click={handleScrollToBottom}
				><img src={ExpandMore} alt="Expand More" /></button
			>
		</div>
	</div>
	<p>
		Powered by SvelteKit + Rust through WebAssembly for fast and secured loudness measurement 🎚️
	</p>
{/if}

<style>
	.analyzers-container > div {
		transition: transform 1000ms;
	}
	.grid {
		transition: transform 0.5s ease;
	}
	.grid.over {
		transform: scale(1.1); /* Enlarges the item slightly */
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2); /* Adds a shadow for depth */
	}
	.controller {
		display: flex;
	}
	.controller-btns {
		display: flex;
		flex-direction: column;
		flex-wrap: nowrap;
		gap: 16px;
	}
	.analyzers-container {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		grid-auto-flow: row;
		gap: 20px;
		width: 100%;
		height: 100%;
		box-sizing: border-box;
		padding-top: 20px;
		padding-left: 60px;
		padding-right: 60px;
		padding-bottom: 48px;
		height: 80vh;
		overflow-y: auto;
		border: 2px solid;
		border-color: antiquewhite;
		scroll-behavior: smooth;
	}
	.analyzer {
		padding-top: 64px;
	}
	button {
		margin-left: 16px;
		padding: 10px;
		cursor: pointer;
		color: #fefefe;
		border-radius: 10px;
		background-color: #222222;
		gap: 16px;
		display: flex;
		margin-right: 4px;
	}

	button:hover {
		background-color: #0f0f0f;
		color: #fefefe;
		border-color: rgb(46, 0, 125);
	}
	#scrollTop,
	#scrollBottom {
		bottom: 48px; /* Position at the bottom of the container */
	}

	#scrollTop {
		right: 10px; /* Position the 'Top' button to the left */
	}

	#scrollBottom {
		bottom: 100px;
		right: 10px;
	}

	button img {
		width: 20px;
		height: 20px;
		display: flex;
		justify-self: center;
	}
	p {
		color: #fefefe;
		display: flex;
		justify-content: center;
	}
	/* Responsive for mobile devices */

	@media (max-width: 1950px) {
		.analyzers-container {
			grid-template-columns: repeat(3, 1fr); /* 3 items in a row on tablet screens */
			transition: all 1s ease-in-out;
		}
	}

	@media (max-width: 1350px) {
		.analyzers-container {
			grid-template-columns: repeat(2, 1fr); /* 2 items in a row on tablet screens */
			transition: all 1s ease-in-out;
		}
	}

	@media (max-width: 880px) {
		.analyzers-container {
			grid-template-columns: 1fr; /* 1 item in a row on mobile screens */
			transition: all 1s ease-in-out;
		}
	}
</style>
