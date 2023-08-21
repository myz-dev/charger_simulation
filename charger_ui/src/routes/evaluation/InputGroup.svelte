<!-- Contains all input elements for the simulation parameters -->
<script lang="ts">
	import type { SimInput } from '$lib/api/fetch_simulation';
	import ExplainedInput from '$lib/components/input/ExplainedInput.svelte';
	import { slide } from 'svelte/transition';

	export let input_done = false;
	export let sim_input: SimInput;
	let step = 0;
	$: {
		if (step > 3) {
			input_done = true;
		}
	}
</script>

<!-- Todo: Overarching form to collect all inputs, mock "fetch" to POST the data to the server. -->
<div class="flex flex-col justify-center items-center gap-3 text-xl h-full">
	<form>
		{#if step === 0}
			<section in:slide>
				<ExplainedInput bind:step>
					<label for="arrival_modifier" slot="title">
						<h3 class="my-0 ml-1">Arrival Modifier</h3>
					</label>
					<div slot="explanation" class="px-1 mb-2">
						<p class="text-lg">
							EVs have a pre-defined probability to arrive at the charging station. <br />
							You can modify this probability with the factor defined here. <br />
						</p>
					</div>
					<div slot="val" class="mb-2">
						<input
							name="arrival_modifier"
							id="arrival_modifier"
							type="number"
							placeholder="Arrival Modifier"
							class="input w-full"
							min="0.2"
							max="2"
							step="0.1"
							bind:value={sim_input.arrival_modifier}
						/>
					</div>
					<div slot="tooltip_content">
						Type in a value between 0.3 and 2.<br />
						0.3 would mean, that your charging stations expect 30% of the usual amount of EVs expected
						to arrive at the charging station. <br /><br />
						Leave empty to not modify the value.
					</div>
				</ExplainedInput>
			</section>
		{/if}
		{#if step === 1}
			<section in:slide>
				<ExplainedInput bind:step>
					<label for="station_count" slot="title">
						<h3 class="my-0 ml-1">Number of stations</h3>
					</label>
					<div slot="explanation" class="px-1 mb-2">
						<p class="text-lg">
							How many stations do you plan to install at your parking lot? <br />
							Don't worry, you can change this value later to compare different alternatives. <br />
						</p>
					</div>
					<div slot="val" class="mb-2">
						<input
							name="station_count"
							id="station_count"
							type="number"
							placeholder="Number of stations"
							class="input w-full"
							min="1"
							step="1"
							bind:value={sim_input.station_count}
						/>
					</div>
					<div slot="tooltip_content">
						Type in a value that is bigger than 1.<br />
						More stations mean you will have an higher upfront investment for your system. <br /><br
						/>
						There will be a "sweet spot" for your case where a specific number of stations gives you
						an optimal return on investment.
					</div>
				</ExplainedInput>
			</section>
		{/if}
		{#if step === 2}
			<section in:slide>
				<ExplainedInput bind:step>
					<label for="battery_consumption" slot="title">
						<h3 class="my-0 ml-1">Car Battery Consumption</h3>
					</label>
					<div slot="explanation" class="px-1 mb-2">
						<p class="text-lg">
							Every EV comes with its own battery and its characteristics. <br />
							To simplify, we will take an average value.
						</p>
					</div>
					<div slot="val" class="mb-2">
						<input
							name="battery_consumption"
							id="battery_consumption"
							type="number"
							placeholder="18kWh/100km"
							class="input w-full"
							min="1"
							step="0.1"
							bind:value={sim_input.battery_consumption}
						/>
					</div>
					<div slot="tooltip_content">
						Type in a value that is bigger than 1.<br />
						18kWh per 100km is a good place to start. <br /><br />
						You can change this value later on to see how it will affect the output of our simulation
						for your case.
					</div>
				</ExplainedInput>
			</section>
		{/if}
		{#if step === 3}
			<section in:slide>
				<ExplainedInput bind:step>
					<label for="station_power" slot="title">
						<h3 class="my-0 ml-1">Station Power</h3>
					</label>
					<div slot="explanation" class="px-1 mb-2">
						<p class="text-lg">
							More power more speed. <br />
							More speed - well, more cost for the facility owner...
						</p>
					</div>
					<div slot="val" class="mb-2">
						<input
							name="station_power"
							id="station_power"
							type="number"
							placeholder="30kW"
							class="input w-full"
							min="1"
							step="0.1"
							bind:value={sim_input.station_power}
						/>
					</div>
					<div slot="tooltip_content">
						Type in a value that is bigger than 1.<br />
						You will have happier customers if they can charge their cars faster at your station.
						<br /><br />
						But you have to watch our for the maximum concurrent power usage of your system, otherwise
						your electricity provider will charge you for the high power spikes your charging stations
						cause.
					</div>
				</ExplainedInput>
			</section>
		{/if}
	</form>
</div>
