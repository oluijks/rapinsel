<script lang="ts">
	import type { FaqsResponse } from '$lib/types';
	import { type CreateQueryResult } from '@tanstack/svelte-query';

	import * as Alert from '$lib/components/ui/alert/index.js';
	import * as Accordion from '$lib/components/ui/accordion/index.js';
	import ExclamationTriangle from 'svelte-radix/ExclamationTriangle.svelte';

	interface Props {
		queryResult: CreateQueryResult<FaqsResponse, Error>;
	}

	let { queryResult }: Props = $props();
</script>

<div>
	{#if $queryResult.status === 'pending'}
		<p class="leading-7 [&:not(:first-child)]:mt-6">Loading faqs...</p>
	{:else if $queryResult.status === 'error'}
		<Alert.Root variant="destructive">
			<ExclamationTriangle class="h-4 w-4" />
			<Alert.Title>Error</Alert.Title>
			<Alert.Description>{$queryResult.error.message}</Alert.Description>
		</Alert.Root>
	{:else}
		<Accordion.Root>
			{#each $queryResult.data.faqs as faq}
				<Accordion.Item value={faq.id}>
					<Accordion.Trigger class="hover:underline-offset-4">{faq.question}</Accordion.Trigger>
					<Accordion.Content>{faq.answer}</Accordion.Content>
				</Accordion.Item>
			{/each}
		</Accordion.Root>
		{#if $queryResult.isFetching}
			<p class="leading-7 [&:not(:first-child)]:mt-6">Fetching faqs...</p>
		{/if}
	{/if}
</div>
