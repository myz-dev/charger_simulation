<script lang="ts">
	import { slide } from 'svelte/transition';
	import Hook from './Hook.svelte';
	import InputGroup from './InputGroup.svelte';
	import Simulate from './Simulate.svelte';
	import { SimInput, SimOutput } from '$lib/api/fetch_simulation';
	import OutputView from './OutputView.svelte';

	let current_state: STATE = 'ON_HOOK';
	let input_done = false;
	let sim_input: SimInput = new SimInput();
	let sim_output: SimOutput = new SimOutput();
	let sim_done = false;

	$: {
		if (input_done) {
			current_state = 'ON_SIMULATE';
		}
	}

	$: {
		if (current_state === 'ON_SIMULATE' && sim_done) {
			current_state = 'ON_OUTPUT';
		}
	}
</script>

<div class="h-full flex flex-col justify-center items-center">
	<!-- Progress
	<ul class="steps steps-horizontal md:steps-vertical">
		<li class="step step-primary">Define</li>
		<li class="step step-primary">Simulate</li>
		<li class="step">Review</li>
		<li class="step">Get Started</li>
	</ul>
	-->
	<div class="h-full w-full">
		{#if current_state === 'ON_HOOK'}
			<div class="h-full m-4" transition:slide>
				<Hook bind:state={current_state} />
			</div>
		{/if}

		{#if current_state === 'ON_INPUT'}
			<div class="h-full m-4" transition:slide>
				<InputGroup bind:input_done bind:sim_input />
			</div>
		{/if}

		{#if current_state === 'ON_SIMULATE'}
			<div class="h-full m-4" transition:slide>
				<Simulate {sim_input} bind:sim_output bind:sim_done />
			</div>
		{/if}

		{#if current_state === 'ON_OUTPUT'}
			<div class="h-full w-full" transition:slide>
				<OutputView {sim_output} />
				<!-- <div>data cards (chartjs) and "get quote" button</div> -->
			</div>
		{/if}
	</div>
</div>
