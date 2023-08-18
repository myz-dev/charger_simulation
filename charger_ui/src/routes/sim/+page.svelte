<script>
	import InputGroup from './InputGroup.svelte';
</script>

<div class="bg-base-100 h-full flex flex-col">
	<!-- Progress -->
	<ul class="steps steps-horizontal md:steps-vertical">
		<li class="step step-primary">Define</li>
		<li class="step step-primary">Simulate</li>
		<li class="step">Review</li>
		<li class="step">Get Started</li>
	</ul>
	<div class="mx-4">
		<InputGroup />
	</div>
</div>
