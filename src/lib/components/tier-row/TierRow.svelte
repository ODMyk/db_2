<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import { Button } from '$lib/components/ui/button';
	import { Check, Edit, MoreHorizontal, Trash, X } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import { Input } from '$lib/components/ui/input';
	import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import {
		validateStringLongerThan,
		validateStringShorterThan,
		validateUnsignedDouble
	} from '$lib/utils';
	import type { Tier } from '$lib';

	export let Id: number;
	export let Title: string;
	export let Price: number;
	export let Limited: boolean;
	export let deletef: () => void;
	export let blocker: { shouldBlock: boolean };
	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;
	export let edit: (t: Tier) => void;

	const errors = { Title: '', Price: '' };

	let newTitle: string;
	let newPrice: string;
	let newLimited: boolean;

	let row: HTMLElement | null | undefined;
	let rowSelected = false;
	let dropdownMenuOpened = false;
	let actualDropdownMenuOpened = false;
	let contextMenuOpened = false;
	let triggerHidden = true;
	let editMode = false;

	const deleteTier = async () => {
		const result = await connectionWrapper(async () => {
			const r: boolean = await invoke('delete_tier', { id: Id });
			return r;
		})();

		if (!result) {
			toast.error(`An arror occured while trying to remove tier with Id ${Id}`);
			return;
		}

		toast.success(`Successfully removed tier with Id ${Id}`);
		deletef();
	};

	const parseTitle = () => {
		return (
			validateStringLongerThan(newTitle, 0, (msg: string) => (errors.Title = msg)) &&
			validateStringShorterThan(newTitle, 51, (msg: string) => (errors.Title = msg))
		);
	};

	const editTier = async () => {
		let price = 0;
		errors.Price = '';
		errors.Title = '';
		const isTitleOk = parseTitle();
		const isPriceOk = validateUnsignedDouble(
			newPrice,
			(n) => (price = n),
			(m) => (errors.Price = m)
		);

		if (!isTitleOk || !isPriceOk) {
			return;
		}

		const result = await connectionWrapper(async () => {
			const r: number = await invoke('edit_tier', {
				tier: {
					id: Id,
					title: newTitle,
					price: price,
					is_limited: newLimited
				}
			});
			return r;
		})();

		if (!result || result < 0) {
			toast.error(`An error occured while trying to edit tier with Id ${Id}`);
			return;
		}

		toast.success(`Successfully edited tier with Id ${Id}`);
		Title = newTitle;
		Limited = newLimited;
		Price = price;
		editMode = false;
		blocker.shouldBlock = false;
		edit({ Title, Limited, Price, Id });
	};

	const openEditDialog = () => {
		blocker.shouldBlock = true;
		editMode = true;
		newTitle = Title;
		newPrice = Price.toString();
		newLimited = Limited;
		setTimeout(() => {
			const overlays = document.getElementsByClassName('backdrop-blur-sm');
			for (const element of overlays) {
				// element.classList.replace('fixed', 'absolute');
				element.classList.replace('inset-0', 'top-30');
				element.classList.replace('bg-background/80', 'bg-background/90');
				element.classList.add('bottom-0', 'left-0', 'right-0', 'w-full', 'h-[95vh]');
				element.addEventListener('click', () => {
					editMode = false;
					blocker.shouldBlock = false;
				});
			}

			const but = document.getElementsByClassName(
				'absolute right-4 top-4 rounded-sm opacity-70'
			)[0];
			but.addEventListener('click', () => {
				editMode = false;
				blocker.shouldBlock = false;
			});
		}, 1);
	};

	onMount(() => {
		row = row?.parentElement?.parentElement;

		row?.addEventListener('mouseenter', () => {
			rowSelected = true;
		});

		row?.addEventListener('mouseleave', () => {
			rowSelected = false;
		});

		window.addEventListener('mousedown', showTrigger);
		window.addEventListener('mouseup', hideTrigger);

		window.addEventListener('keydown', (e) => {
			if (e.keyCode === 27) {
				blocker.shouldBlock = false;
			}
		});
	});

	const showTrigger = (event: MouseEvent) => {
		if (event.button === 2) {
			contextMenuOpened = false;
			triggerHidden = false;
			actualDropdownMenuOpened = false;
			setTimeout(() => (dropdownMenuOpened = actualDropdownMenuOpened), 100);
		}
	};

	const hideTrigger = (event: MouseEvent) => {
		if (event.button === 2) {
			triggerHidden = true;
		}
	};
</script>

<Dialog.Root
	bind:open={editMode}
	closeOnOutsideClick={false}
	onOpenChange={() => (blocker.shouldBlock = editMode)}
>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Edit tier entry</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Id</div>
				<div class="flex flex-col">
					<div class="flex items-center h-12 text-sm">Title</div>
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col">
					<div class="flex items-center h-12 text-sm">Price</div>
					<div>
						{#if errors.Price}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex items-center h-12 text-sm">Limited</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<Input disabled value={Id} class="h-12 bg-backgroundSecondary" />
				<div class="flex flex-col">
					<Input bind:value={newTitle} class="h-12 bg-backgroundSecondary" />
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">{errors.Title}</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col">
					<Input bind:value={newPrice} class="h-12 bg-backgroundSecondary" />
					<div>
						{#if errors.Price}
							<span class="text-xs text-red-600">{errors.Price}</span>
						{/if}
					</div>
				</div>
				<div class="h-12 flex items-center">
					<Switch bind:checked={newLimited} class="data-[state=unchecked]:bg-backgroundSecondary" />
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={editTier}
				><Check size={24} /></Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ContextMenu.Root bind:open={contextMenuOpened}>
	<ContextMenu.Trigger asChild let:builder>
		<Table.Row class="cursor-default">
			<Table.Cell>
				<span bind:this={row} class="select-text">{Id}</span>
			</Table.Cell>
			<Table.Cell>
				<span class="select-text">{Title}</span>
			</Table.Cell>
			<Table.Cell>
				<span class="select-text">{Price}</span>
			</Table.Cell>
			<Table.Cell class="p-0">
				<div class="flex w-full justify-center">
					<Checkbox disabled checked={Limited} class="disabled:hover:cursor-default" />
				</div>
			</Table.Cell>

			<Table.Cell>
				<DropdownMenu.Root
					bind:open={actualDropdownMenuOpened}
					onOpenChange={() =>
						setTimeout(
							() => {
								dropdownMenuOpened = actualDropdownMenuOpened;
							},
							actualDropdownMenuOpened ? 100 : 0
						)}
				>
					<DropdownMenu.Trigger asChild let:builder>
						<div class="bg-transparent w-[32px] h-[8px] flex items-center justify-center">
							<Button
								builders={[builder]}
								class="size-5 p-0 m-0 transition-opacity delay-150 {!rowSelected &&
								!dropdownMenuOpened
									? 'hidden'
									: ''}"><MoreHorizontal size={12} /></Button
							>
						</div>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content
						style="min-width: 0;"
						class=" w-[100px] p-0 border-third rounded-none bg-background"
					>
						<DropdownMenu.Item
							class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
							on:click={openEditDialog}
						>
							<Edit class="mr-2 size-4" />
							<span>Edit</span>
						</DropdownMenu.Item>
						<DropdownMenu.Item
							class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
							on:click={deleteTier}
						>
							<Trash class="mr-2 size-4 stroke-red-600" />
							<span>Remove</span>
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</Table.Cell>

			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				use:builder.action
				{...builder}
				class="absolute left-0 w-full h-1/3 {triggerHidden ? 'hidden' : ''}"
			/>
		</Table.Row>
	</ContextMenu.Trigger>
	<ContextMenu.Content
		style="min-width: 0;"
		class=" w-[100px] p-0 border-third rounded-none bg-background"
	>
		<ContextMenu.Item
			class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
			on:click={openEditDialog}
		>
			<Edit class="mr-2 size-4" />
			<span>Edit</span>
		</ContextMenu.Item>
		<ContextMenu.Item
			class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
			on:click={deleteTier}
		>
			<Trash class="mr-2 size-4 stroke-red-600" />
			<span>Remove</span></ContextMenu.Item
		>
	</ContextMenu.Content>
</ContextMenu.Root>
