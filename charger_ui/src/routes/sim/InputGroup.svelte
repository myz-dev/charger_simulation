<!-- Contains all input elements for the simulation parameters -->
<script lang="ts">
	import ExplainedInput from '$lib/components/input/ExplainedInput.svelte';
</script>

<div class="flex flex-col justify-between gap-3 text-xl">
	<div class="bg-base-300 py-4 px-3 rounded-md">
		<ExplainedInput>
			<div slot="val">
				<input
					type="number"
					placeholder="Arrival Modifier"
					class="input w-full"
					min="0.2"
					max="2"
					step="0.1"
				/>
			</div>
			<div slot="explanation">
				EVs have a pre-defined probability to arrive at the charging station. <br /><br />
				You can modify this probability with the factor defined here. <br />
				Please type in a value between 0,3 and 2. <br /><br />
				Leave empty to not modify the value.
			</div>
		</ExplainedInput>
	</div>

	<div>Stations</div>
	<!-- Advanced -->
	<div>Car consumption</div>
	<div>Charging power</div>
</div>
