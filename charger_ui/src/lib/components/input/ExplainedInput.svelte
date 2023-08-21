<!-- Input box with explanatory text, validation and additional info tooltip.
	 Use the `title` slot for the title of the input element.
	 Use the `explanation` slot for the explanatory text.
     Use the `val` slot to insert the input element.
     Use the `anchor` slot to insert the tooltip anchor
     Use the `tooltip_content` slot for the tooltip content.
  -->
<script lang="ts">
	import ArrowDownwardAltSvg from '../icons/ArrowDownwardAltSVG.svelte';
	import InfoSvg from '../icons/InfoSVG.svelte';
	import Tooltip from '../tooltip/Tooltip.svelte';
	export let step: number;
</script>

<div class="flex flex-col relative prose bg-base-200 rounded-lg p-4 shadow-md text-base-content">
	<slot name="title">
		<h3 class="my-0 ml-1">Arrival Modifier</h3>
	</slot>
	<div>
		<slot name="explanation">
			<p>This is some explanatory text.</p>
		</slot>
	</div>
	<div class="flex flex-row justify-between items-center gap-3">
		<div class="grow">
			<slot name="val">
				<input type="text" placeholder="Type here" class="input w-full max-w-xs" />
			</slot>
		</div>
		<div class="">
			<Tooltip tailwind_classes={['text-left', 'rounded-lg', 'px-4']} bg_color_class={'bg-neutral'}>
				<div slot="anchor" class="h-7 w-7 text-base-content">
					<InfoSvg />
				</div>
				<div
					slot="tooltip"
					class="bg-neutral text-neutral-content border-neutral-focus rounded-lg"
					style="min-width: 300px;"
				>
					<p class="">
						<slot name="tooltip_content">Some explanation</slot>
					</p>
				</div>
			</Tooltip>
		</div>
	</div>
	<div class="absolute left-1/2 -translate-x-1/2 bottom-0 translate-y-1/2">
		<button
			on:click={() => (step += 1)}
			class="bounce_btn bg-primary rounded-md hover:bg-primary-content text-primary-content hover:text-primary"
		>
			<div class="h-8 w-8">
				<ArrowDownwardAltSvg />
			</div>
		</button>
	</div>
</div>

<style>
	.bounce_btn {
		animation: bounce 2s ease;
	}
	/* see https://codepen.io/nelledejones/pen/gOOPWrK */
	@keyframes bounce {
		70% {
			transform: translateY(0%);
		}
		60% {
			transform: translateY(-10%);
		}
		90% {
			transform: translateY(0%);
		}
		95% {
			transform: translateY(-7%);
		}
		97% {
			transform: translateY(0%);
		}
		99% {
			transform: translateY(-3%);
		}
		100% {
			transform: translateY(0);
		}
	}
</style>
