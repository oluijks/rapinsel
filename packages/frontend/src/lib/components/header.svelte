<script>
	import { page } from '$app/stores';
	import { Button } from '$lib/components/ui/button';
	import ThemeSwitcher from '$lib/components/theme-switcher.svelte';
</script>

<header
	class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
	<div class="container flex h-14 max-w-screen-xl items-center px-3 md:px-4 xl:px-0">
		<div>
			<nav>
				<Button href="/" variant={$page.url.pathname === '/' ? 'outline' : 'link'}>Home</Button>
				<Button href="/faqs" variant={$page.url.pathname === '/faqs' ? 'outline' : 'link'}
					>Faqs</Button
				>
			</nav>
		</div>
		<div class="flex flex-1 items-center justify-end space-x-2">
			<ThemeSwitcher />
		</div>
	</div>
</header>
