import type { FaqsResponse } from './types';

export const api = (customFetch = fetch) => ({
	fetchFaqs: async () => {
		const resp = await customFetch('http://192.168.1.93:5174/api/v1/faqs');
		const data: FaqsResponse = await resp.json();

		return data;
	}
});
