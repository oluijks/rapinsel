import type { FaqsResponse } from './types';

export const api = (customFetch = fetch) => ({
	fetchFaqs: async () => {
		const resp = await customFetch('http://127.0.0.1:5174/api/v1/faqs');
		const data: FaqsResponse = await resp.json();

		return data;
	}
});
