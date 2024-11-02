<script lang="ts">
	import type { PageData } from './$types';

	import { QueryClientProvider } from '@tanstack/svelte-query';
	import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools';

	import '../assets/styles/app.css';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner';

	import Header from '$lib/components/header.svelte';
	import TailwindIndicator from '$lib/components/tailwind-indicator.svelte';

	interface Props {
		data: PageData;
		children?: import('svelte').Snippet;
	}

	let { data, children }: Props = $props();
</script>

<QueryClientProvider client={data.queryClient}>
	<main>
		<Header />
		{@render children?.()}
		<Toaster />
		<ModeWatcher defaultMode="dark" />
		<TailwindIndicator />
	</main>
	<SvelteQueryDevtools />
</QueryClientProvider>
