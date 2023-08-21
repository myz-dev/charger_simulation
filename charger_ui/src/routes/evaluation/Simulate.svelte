<!-- Todo: Simulation animation -->
<script lang="ts">
	import { post_sim_request, SimOutput, type SimInput } from '$lib/api/fetch_simulation';
	import { onMount } from 'svelte';

	export let sim_input: SimInput;
	export let sim_output: SimOutput;
	export let sim_done: boolean;

	let response: Promise<SimOutput>;
	onMount(async () => {
		try {
			response = post_sim_request(sim_input);
			sim_output = await response;
			sim_done = true;
		} catch (error) {
			//TODO: Error handling
			console.log("Todo: Error handling. Error: " + error)
		}
	});
</script>

{#await response}
	<div>sim animation</div>
{:then data}
	<div>
		here is the data: {data}
	</div>
{:catch e}
	<div class="h-full flex flex-col justify-center items-center">
		<div class="flex flex-col bg-error text-error-content rounded-md p-4 prose">
			<h2>Oops! An error happened!</h2>
			<p>
				<!-- {e} -->
				For the purposes of this demo, we can continue using some mock values for the visualization.
			</p>
			<div class="flex flex-row justify-end">
				<button
					on:click={() => {
						sim_output = SimOutput.produce_mock_output();
						sim_done = true;
					}}
					class="btn btn-primary"
				>
					Ignore error!
				</button>
			</div>
		</div>
	</div>
{/await}
