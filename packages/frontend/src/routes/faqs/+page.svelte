<script lang="ts">
	import { api } from '$lib/api';
	import Faqs from './components/Faqs.svelte';
	import type { FaqsResponse } from '$lib/types';
	import { createQuery } from '@tanstack/svelte-query';

	const queryResult = createQuery<FaqsResponse, Error>({
		queryKey: ['fetchFaqs'],
		queryFn: () => api().fetchFaqs()
	});
</script>

<div class="container mt-10 max-w-screen-xl px-6 md:px-4 xl:px-0">
	<h1 class="mb-8 scroll-m-20 text-4xl lg:text-5xl">Faqs</h1>
	<Faqs {queryResult} />
</div>
