<script lang="ts">
	import { SimOutput } from '$lib/api/fetch_simulation';
	import FinancialOverview from '$lib/components/charts/FinancialOverview.svelte';
	import PeakPowerBars from '$lib/components/charts/PeakPowerBars.svelte';
	import { onMount } from 'svelte';

	export let sim_output: SimOutput;

	onMount(() => {
		//TODO: Delete after debugging
		sim_output = SimOutput.produce_mock_output();
	});
</script>

<div class="my-2 w-full flex flex-col justify-center items-center gap-3">
	<div class="">
		<PeakPowerBars
			actual_peak_kW={sim_output.actual_max_kW}
			theoretical_peak_kW={sim_output.theoretical_max_kW}
			average_peak_kW={sim_output.actual_max_kW * 0.7}
		/>
	</div>
	<div class="">
		<FinancialOverview />
	</div>
	<div class="">
		<FinancialOverview />
	</div>
</div>
