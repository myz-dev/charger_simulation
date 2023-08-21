<!-- Sell the customer on the value of the simulation and the app they are using -->
<script lang="ts">
	import ArrowRightAltSvg from '$lib/components/icons/ArrowRightAltSVG.svelte';

	export let state: STATE;
</script>

<div class="h-full flex flex-col justify-center items-center">
	<div
		class="prose bg-base-200 rounded-lg p-4 shadow-md flex flex-col
	text-base-content"
	>
		<h3 class="text-center">Don't worry, this will be a short interview.</h3>
		<p>
			We have only four questions for you.<br />
			Based on those we will present you with all the data you could ask for, when planning out the electrification
			of your parking space.
			<!--
				First we will ask you only four questions to get an idea of what kind of space we are dealing
				with, then we will present you with all the data you could wish for when deciding upon your next
				steps. And if you should wish to install charging stations on your premises, we can navigate you
				through that process and save you all the hassle this undertaking would bring with it.
			-->
		</p>
		<button
			on:click={() => (state = 'ON_INPUT')}
			class="btn btn-primary flex flex-row justify-between mb-2"
		>
			<p>Let's start then!</p>
			<div class="h-5 w-5 rotate-90"><ArrowRightAltSvg /></div>
		</button>
	</div>
</div>
