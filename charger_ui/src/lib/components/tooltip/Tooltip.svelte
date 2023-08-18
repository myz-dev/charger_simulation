<!-- Tooltip container -->
<!-- Use the `anchor` slot to register the parent and the `tooltip` slot to fill in the tooltip to open when hovering
     the parent. -->
<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { make_uid } from '$lib/utilities';
	import { de_register_events, register_events } from './manage_tooltip';

	export let tailwind_classes: string[] = ['p-4', 'text-center'];
	export let bg_color_class: string = 'bg-neutral';

	let anchor: HTMLDivElement;
	let tooltip: HTMLDivElement;
	let tooltip_arrow: HTMLDivElement;

	const id = make_uid();

	onMount(() => register_events(tooltip, anchor, tooltip_arrow));
	onDestroy(() => de_register_events(tooltip, anchor, tooltip_arrow));
</script>

<div class="anchor" bind:this={anchor} aria-describedby={id}>
	<slot name="anchor" />
</div>

<div
	{id}
	class="tooltip {bg_color_class} {tailwind_classes.join(' ')}"
	bind:this={tooltip}
	role="tooltip"
>
	<slot name="tooltip" />
	<div class="arrow {bg_color_class}" bind:this={tooltip_arrow} />
</div>

<style>
	.tooltip {
		display: none;
		position: absolute;
		left: 0;
		top: 0;
	}

	.arrow {
		position: absolute;

		width: 8px;
		height: 8px;
		transform: rotate(45deg);
	}
</style>
