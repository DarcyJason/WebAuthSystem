<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Eye, EyeOff } from '@lucide/svelte';

	let { value = $bindable(), id, placeholder = '••••••••', required = false } = $props<{
		value: string;
		id?: string;
		placeholder?: string;
		required?: boolean;
	}>();

	let show = $state(false);
</script>

<div class="relative">
	<Input {id} type={show ? 'text' : 'password'} {placeholder} {required} bind:value class="pr-10" />
	<button
		type="button"
		onclick={() => (show = !show)}
		class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
		aria-label={show ? 'Hide password' : 'Show password'}
	>
		{#if show}
			<EyeOff size={16} />
		{:else}
			<Eye size={16} />
		{/if}
	</button>
</div>
