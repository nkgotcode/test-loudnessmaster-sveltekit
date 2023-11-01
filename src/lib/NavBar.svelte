<script>
	import { onMount } from 'svelte';

	let lastScrollTop = 0;
	let navBar;

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
	<h1>LoudnessMaster</h1>
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
</style>
