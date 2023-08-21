<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Chart,
		LineController,
		LineElement,
		CategoryScale,
		LinearScale,
		Tooltip
	} from 'chart.js';
	import { ThemeColors } from './get_theme_colors';
	// export let station_count: number;

	let canvas: HTMLCanvasElement;
	let chart: Chart;
	onMount(() => {
		const colors = ThemeColors.get_current_colors();

		Chart.register(LineController, LineElement, CategoryScale, LinearScale, Tooltip);
		Chart.defaults.color = `hsl(${colors.base_content})`;

		chart = new Chart(canvas, {
			type: 'bar',
			data: {
				labels: ['1Y', '2Y', '3Y', '4Y', '5Y', '6Y', '7Y', '8Y'],
				datasets: [
					{
						label: 'Debt Repayment',
						data: [40, 38, 34, 28, 22, 16, 10, 8],
						backgroundColor: `hsl(${colors.primary_color})`,
						hoverBackgroundColor: `hsl(${colors.primary_focus_color})`
					},
					{
						label: 'Energy Bill',
						data: [60, 61, 64, 68, 71, 75, 79, 75],
						backgroundColor: `hsl(${colors.secondary_color})`,
						hoverBackgroundColor: `hsl(${colors.secondary_focus_color})`
					},
					{
						label: 'Profit',
						data: [0, 1, 2, 4, 7, 9, 11, 17],
						backgroundColor: `hsl(${colors.accent_color})`,
						hoverBackgroundColor: `hsl(${colors.accent_focus_color})`
					}
				]
			},
			options: {
				responsive: true,
				scales: {
					x: {
						stacked: true
					},
					y: {
						stacked: true
						// title: {
						// 	display: true,
						// 	text: '%',
						// 	align: 'center'
						// }
					}
				}
			}
		});
	});
</script>

<div class="bg-base-200 p-4 prose flex flex-col justify-start items-center rounded-md">
	<div class="mb-3">
		<h3 class="m-0">Cashflow</h3>
	</div>
	<div class="relative">
		<canvas bind:this={canvas} />
	</div>
</div>
