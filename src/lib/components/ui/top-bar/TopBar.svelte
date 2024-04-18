<script lang="ts">
	import { HelpCircle, Minus, Moon, Sun, X } from 'lucide-svelte';
	import { toggleMode, mode } from 'mode-watcher';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Accordion from '$lib/components/ui/accordion';
	import { appWindow } from '@tauri-apps/api/window';

	export let Title: string = 'Hello World';

	let showHelp = false;

	const startDrag = async (event: MouseEvent) => {
		await appWindow.startDragging();
	};
</script>

<Dialog.Root bind:open={showHelp} closeOnOutsideClick={false}>
	<Dialog.Content
		style="z-index:100;"
		class="bg-backgroundSecondary border-none"
		on:click={(event) => event.preventDefault()}
	>
		<Dialog.Header>
			<Dialog.Title>Help</Dialog.Title>
		</Dialog.Header>
		<Accordion.Root class="w-full border-spacing-0 border-collapse">
			<Accordion.Item value="item-1">
				<Accordion.Trigger>About application</Accordion.Trigger>
				<Accordion.Content
					>This application was created in 2024 using Tauri + SvelteKit + TailwindCSS + PostgreSQL
					stack. It allows you to view, create, edit and delete data from specific database using
					modern UI</Accordion.Content
				>
			</Accordion.Item>
			<Accordion.Item value="item-2">
				<Accordion.Trigger>Themes</Accordion.Trigger>
				<Accordion.Content>
					You can change theme (light/dark) using the left button on the topbar of application
				</Accordion.Content>
			</Accordion.Item>
			<Accordion.Item value="item-3">
				<Accordion.Trigger>Hotkeys</Accordion.Trigger>
				<Accordion.Content>Here will be information about hotkeys</Accordion.Content>
			</Accordion.Item>
			<Accordion.Item value="item-4">
				<Accordion.Trigger>About author</Accordion.Trigger>
				<Accordion.Content>
					<h3>Dmytro Ostapenko</h3>
					<p>github.com/ODMyk</p>
					<p>Junior Web-Developer looking for a job</p>
				</Accordion.Content>
			</Accordion.Item>
		</Accordion.Root>
	</Dialog.Content>
</Dialog.Root>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="h-[25px] w-screen bg-third grid grid-cols-3 items-center justify-center select-none cursor-default p-0 z-50"
	on:mousedown={startDrag}
>
	<div class="w-full h-[80%] flex items-center justify-start space-x-2">
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			class="items-center flex justify-center h-full ml-2 cursor-pointer"
			on:click={toggleMode}
			on:mousedown={(e) => e.stopImmediatePropagation()}
		>
			{#if $mode === 'dark'}
				<Sun size={16} />
			{:else}
				<Moon size={16} />
			{/if}
		</div>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			class="items-center justify-center flex h-full cursor-pointer"
			on:click={() => {
				showHelp = true;
				setTimeout(() => {
					const overlays = document.getElementsByClassName('bg-background/80');
					for (const element of overlays) {
						// element.classList.replace('fixed', 'absolute');
						element.classList.replace('inset-0', 'top-30');
						element.classList.replace('bg-background/80', 'bg-background/90');
						element.classList.add('bottom-0', 'left-0', 'right-0', 'w-full', 'h-[95vh]');
						element.addEventListener('click', () => {
							showHelp = false;
						});
					}
				}, 1);
			}}
			on:mousedown={(e) => e.stopImmediatePropagation()}
		>
			<HelpCircle size="16" />
		</div>
	</div>
	<div class="justify-center items-center flex w-full h-full text-sm">
		<h3>{Title}</h3>
	</div>
	<div class="p-0 flex w-full h-full justify-end items-center">
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			class="hover:bg-backgroundSecondary h-full w-[12.5%] flex items-center justify-center"
			on:click={async () => {
				await appWindow.minimize();
			}}
			on:mousedown={(e) => e.stopImmediatePropagation()}
		>
			<Minus size={16} />
		</div>

		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			class="hover:bg-red-600 h-full w-[12.5%] flex items-center justify-center"
			on:click={() => window.close()}
			on:mousedown={(e) => e.stopImmediatePropagation()}
		>
			<X size={16} />
		</div>
	</div>
</div>
