<script lang="ts">
	import { HelpCircle, Minus, Moon, Sun, X } from 'lucide-svelte';
	import { toggleMode, mode } from 'mode-watcher';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Accordion from '$lib/components/ui/accordion';
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { shell } from '@tauri-apps/api';

	export let Title: string = 'Hello World';
	export let settings: { hotkey: (e: KeyboardEvent) => void };

	let showHelp = false;
	let hotkey = (e: KeyboardEvent) => {};

	const startDrag = async (event: MouseEvent) => {
		await appWindow.startDragging();
	};

	const a = (e: KeyboardEvent) => {
		if (e.keyCode === 27 && showHelp) {
			showHelp = false;
			settings.hotkey = hotkey;
			window.removeEventListener('keydown', a);
		}
	};

	const showHelpWindow = () => {
		if (!showHelp) {
			hotkey = settings.hotkey;
			settings.hotkey = () => {};
		}
		showHelp = true;
		setTimeout(() => {
			window.addEventListener('keydown', a);
			const overlays = document.getElementsByClassName('backdrop-blur-sm');
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
	};

	onMount(() => {
		window.addEventListener('keydown', (e) => {
			if (e.ctrlKey && (e.key === 'h' || e.key === 'р')) {
				showHelpWindow();
			}

			if (e.ctrlKey && (e.key === 't' || e.key === 'е')) {
				toggleMode();
			}

			if (showHelp) {
				e.stopImmediatePropagation();
			}
		});
	});
</script>

<Dialog.Root
	bind:open={showHelp}
	closeOnOutsideClick={false}
	onOpenChange={() => {
		settings.hotkey = hotkey;
	}}
>
	<Dialog.Overlay style="z-index:99;" />
	<Dialog.Portal style="z-index: 99;" />
	<Dialog.Content
		style="z-index:100;"
		class="bg-backgroundSecondary border-none"
		on:click={(event) => event.preventDefault()}
	>
		<Dialog.Header>
			<Dialog.Title>Help</Dialog.Title>
		</Dialog.Header>
		<Accordion.Root class="w-full border-spacing-0 border-collapse divide-none">
			<Accordion.Item value="item-1" class="border-none">
				<Accordion.Trigger>About application</Accordion.Trigger>
				<Accordion.Content
					>This application was created in 2024 using Tauri + SvelteKit + TailwindCSS stack. It
					allows you to view, create, edit and delete data from specific database using modern UI</Accordion.Content
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
				<Accordion.Content>
					<p>You can open help window with Ctrl+H</p>
					<p>You can change theme using Ctrl+T</p>
					<p>
						On the tabs Tiers, Users, Songs, Playlists and Playlist Songs you may press Ctrl+N in
						order to open create
					</p>
					<p>dialog</p>
					<p>You can navigate through the tabs using Ctrl+(1-6)</p>
					<p>On the SQL tab you can navigate through queries using Alt+(1-8)</p>
				</Accordion.Content>
			</Accordion.Item>
			<Accordion.Item value="item-4">
				<Accordion.Trigger>About author</Accordion.Trigger>
				<Accordion.Content>
					<h3>Dmytro Ostapenko</h3>
					<p>Junior Web-Developer looking for a job</p>
					<p>
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<!-- svelte-ignore a11y-no-static-element-interactions -->
						<span
							class="text-indigo-500 cursor-pointer select-none hover:underline"
							on:click={() => shell.open('https://www.github.com/ODMyk')}
						>
							Github Page
						</span>
					</p>
				</Accordion.Content>
			</Accordion.Item>
		</Accordion.Root>
	</Dialog.Content>
</Dialog.Root>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="h-[25px] w-screen bg-third grid grid-cols-3 items-center justify-center select-none cursor-default p-0"
	on:mousedown={startDrag}
	style="z-index: 101;"
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
			on:click={showHelpWindow}
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
			on:click={async () => await appWindow.close()}
			on:mousedown={(e) => e.stopImmediatePropagation()}
		>
			<X size={16} />
		</div>
	</div>
</div>
