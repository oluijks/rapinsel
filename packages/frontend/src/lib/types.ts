export type Faq = {
	id: string;
	question: string;
	answer: string;
	createdAt: Date;
	updatedAt: Date;
};

export type FaqResponse = {
	status: string;
	faq: Faq;
};

export type FaqsResponse = {
	status: string;
	faqs: Faq[];
};
