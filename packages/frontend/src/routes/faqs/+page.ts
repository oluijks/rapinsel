import { api } from '$lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = (async ({ parent, fetch }) => {
	const { queryClient } = await parent();

	await queryClient.prefetchQuery({
		queryKey: ['fetchFaqs', fetch],
		queryFn: () => api(fetch).fetchFaqs()
	});
}) satisfies PageLoad;
