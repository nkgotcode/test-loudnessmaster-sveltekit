<script>
	import { dontShowAgain } from './store';
	export let showModal;
	export let onClose;

	let localDontShowAgain = false;

	function handleClose() {
		onClose();
		dontShowAgain.set(localDontShowAgain); // Update the store when closing the modal
	}
</script>

{#if showModal}
	<div class="modal">
		<div class="modal-content">
			<p>
				Disclaimer: Online processing will send your data to our servers, but will not store your
				audio. Currently only support 16 and 32 bit WAV file for online processing. For accurate 24
				bit audio measurement, please use offline processing. In general, offline will give better
				performance and more stable results than online.
			</p>
			<label>
				<input type="checkbox" bind:checked={localDontShowAgain} />
				Do not show this message again
			</label>
			<button on:click={handleClose}>Close</button>
		</div>
	</div>
{/if}

<style>
	/* Add your modal styling here */
	.modal {
		background-color: #fefefe;
		padding-top: 64px;
		height: max-content;
		left: 50%;
		position: absolute; /* Position it absolutely within a relative container */
		transform: translateX(-50%); /* Adjust horizontal position */
		z-index: 10;
	}
</style>
