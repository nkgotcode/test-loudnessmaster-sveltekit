<script>
	import { onMount } from 'svelte';
	import Switch from './Switch.svelte';
	import { fade, fly } from 'svelte/transition';
	import { quadIn, quadOut, sineInOut } from 'svelte/easing';
	import { processingType } from './store';
	let lastScrollTop = 0;
	let navBar;
	let multiValue = 'Offline';

	$: if (multiValue === 'Online') {
		processingType.set('Online');
	} else {
		processingType.set('Offline');
	}

	onMount(() => {
		window.addEventListener('scroll', function () {
			let scrollTop = window.scrollY || document.documentElement.scrollTop;

			if (scrollTop > lastScrollTop) {
				navBar.style.transform = 'translateY(-100%)';
			} else {
				navBar.style.transform = 'translateY(0)';
			}

			lastScrollTop = scrollTop;
		});
	});
</script>

<div bind:this={navBar} class="navBar">
	{#if multiValue == 'Online'}
		<h1>
			LoudnessMaster
			<span
				class="live-text"
				in:fly={{ x: 20, duration: 400, easing: sineInOut, opacity: 0.5 }}
				out:fly={{ x: -20, duration: 200, easing: quadIn, opacity: 0.5 }}>live</span
			>
		</h1>
	{:else}
		<h1
			in:fly={{ x: 20, duration: 400, easing: sineInOut, opacity: 0.5 }}
			out:fly={{ x: -20, duration: 200, easing: quadIn, opacity: 0.5 }}
		>
			LoudnessMaster
		</h1>
	{/if}
	<div class="switch">
		<Switch
			bind:value={multiValue}
			label="Processing"
			design="multi"
			options={['Offline', 'Online']}
			fontSize={12}
		/>
		{$processingType}
		<!-- <p>
			{multiValue}
		</p> -->
	</div>
</div>

<style>
	.navBar {
		display: flex;
		justify-content: center;
		align-items: center;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 60px;
		background-color: #0f0f0f;
		z-index: 100;
		transition: transform 0.3s ease;
	}

	h1 {
		margin: 0;
		padding: 0px;
		text-align: center;
		color: #fefefe;
		font-weight: normal;
		font-size: 3.5vw;
	}
	.switch {
		color: #fefefe;
		position: fixed;
		right: 0;
	}
	.live-text {
		font-size: 1rem; /* Adjust font size as needed */
		color: #d53131; /* Optional: different color for visibility */
	}
</style>
