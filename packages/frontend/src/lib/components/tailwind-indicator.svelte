<!--
 @component
 Utility component for showing tailwind breakpoints

 - Usage:

```svelte
<TailwindIndicator />
```
-->

<div
	class="font-mono fixed bottom-1 left-1 z-50 flex h-6 w-6 items-center justify-center rounded-full bg-transparent p-3 text-xs text-white"
>
	<div class="block sm:hidden">xs</div>
	<div class="hidden sm:block md:hidden lg:hidden xl:hidden 2xl:hidden">sm</div>
	<div class="hidden md:block lg:hidden xl:hidden 2xl:hidden">md</div>
	<div class="hidden lg:block xl:hidden 2xl:hidden">lg</div>
	<div class="hidden xl:block 2xl:hidden">xl</div>
	<div class="3xl:hidden hidden 2xl:block">2xl</div>
	<div class="3xl:block hidden">3xl</div>
</div>
