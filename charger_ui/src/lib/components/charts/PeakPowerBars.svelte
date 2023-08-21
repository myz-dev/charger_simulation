<script lang="ts">
	import { onMount } from 'svelte';
	import { Chart, BarController, BarElement, CategoryScale, LinearScale, Tooltip } from 'chart.js';
	import { ThemeColors } from './get_theme_colors';
	// import colorNames from "daisyui/src/theming/colorNames"
	export let actual_peak_kW: number;
	export let theoretical_peak_kW: number;
	export let average_peak_kW: number; // this is just a mock value
	let canvas: HTMLCanvasElement;
	let chart: Chart;

	onMount(() => {
		const colors = ThemeColors.get_current_colors();
		Chart.register(BarController, BarElement, CategoryScale, LinearScale, Tooltip);
		Chart.defaults.color = `hsl(${colors.base_content})`;

		chart = new Chart(canvas, {
			type: 'bar',
			data: {
				labels: ['Theoretical Peak Power', 'Peak Power', 'Average Power'],
				datasets: [
					{
						data: [theoretical_peak_kW, actual_peak_kW, average_peak_kW],
						label: 'First set',
						backgroundColor: `hsl(${colors.primary_color})`,
						barPercentage: 0.6,
						hoverBackgroundColor: `hsl(${colors.primary_focus_color})`
					}
				]
			},
			options: {
				indexAxis: 'y',
				maintainAspectRatio: true,
				responsive: true,
				scales: {
					x: {
						title: {
							align: 'center',
							display: true,
							text: '[kW]'
						}
					}
				}
			}
		});
	});
</script>

<div class="bg-base-200 p-4 prose flex flex-col justify-start items-center rounded-md">
	<div class="mb-3">
		<h3 class="m-0">Power at a glance</h3>
	</div>
	<div class="relative">
		<canvas bind:this={canvas} />
	</div>
</div>
