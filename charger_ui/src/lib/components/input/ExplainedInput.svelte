<!-- Input variable with explanation tooltip and validation.
     Use the `val` slot to insert the input element -->
<script lang="ts">
	import InfoSvg from '../icons/InfoSVG.svelte';
	import Tooltip from '../tooltip/Tooltip.svelte';
</script>

<div class="flex flex-row justify-between items-center gap-3">
	<div class="grow">
		<slot name="val">
			<input type="text" placeholder="Type here" class="input w-full max-w-xs" />
		</slot>
	</div>
	<div class="">
		<Tooltip tailwind_classes={['text-left', 'rounded-lg', 'p-2']} bg_color_class={'bg-neutral'}>
			<div slot="anchor" class="h-7 w-7">
				<InfoSvg />
			</div>
			<div
				slot="tooltip"
				class="bg-neutral text-neutral-content border-neutral-focus rounded-lg px-3 py-2"
			>
				<p class="">
					<slot name="explanation">Some explanation</slot>
				</p>
			</div>
		</Tooltip>
	</div>
</div>
