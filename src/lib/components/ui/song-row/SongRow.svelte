<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import { Button } from '$lib/components/ui/button';
	import { Check, Edit, MoreHorizontal, X } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import Input from '../input/input.svelte';

	export let Id: number;
	export let Title: string;
	export let UserId: number;
	export let TimesPlayed: number;

	let newTitle: string;
	let newUserId: number;
	let newTimesPlayed: number;

	let row: HTMLElement | null | undefined;
	let rowSelected = false;
	let dropdownMenuOpened = false;
	let triggerHidden = true;
	let editMode = false;

	const deleteSong = async () => {
		const result: { 0: string; 1: boolean } = await invoke('delete_song', { id: Id });
		let id: string | number = 0;
		if (result[1]) {
			id = toast.info(result[0]);
		} else {
			id = toast.error(result[0]);
		}
		setTimeout(() => toast.dismiss(id), 1600);
	};

	const editSong = () => {
		// api-call here
		editMode = false;
	};

	const openEditDialog = () => {
		editMode = true;
		newTitle = Title;
		newUserId = UserId;
		newTimesPlayed = TimesPlayed;
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
	});

	const showTrigger = (event: MouseEvent) => {
		if (event.button === 2) {
			triggerHidden = false;
		}
	};

	const hideTrigger = (event: MouseEvent) => {
		if (event.button === 2) {
			triggerHidden = true;
		}
	};
</script>

<Dialog.Root bind:open={editMode}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Edit song entry</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Id</div>
				<div class="flex items-center h-12 text-sm">Title</div>
				<div class="flex items-center h-12 text-sm">UserId</div>
				<div class="flex items-center h-12 text-sm">Times Played</div>
			</div>
			<div class="col-span-3 space-y-4">
				<Input disabled value={Id} class="h-12 bg-backgroundSecondary" />
				<Input bind:value={newTitle} class="h-12 bg-backgroundSecondary" />
				<Input bind:value={newUserId} class="h-12 bg-backgroundSecondary" />
				<Input bind:value={newTimesPlayed} class="h-12 bg-backgroundSecondary" />
			</div>
		</div>

		<Dialog.Footer>
			<Button
				variant="outline"
				class="bg-third hover:bg-backgroundSecondary"
				type="submit"
				on:click={editSong}><Check size={24} /></Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ContextMenu.Root>
	<ContextMenu.Trigger asChild let:builder>
		<Table.Row class="cursor-default">
			<Table.Cell>
				<span bind:this={row} class="select-text">{Id}</span>
			</Table.Cell>
			<Table.Cell>
				<span bind:this={row} class="select-text">{Title}</span>
			</Table.Cell>
			<Table.Cell>
				<span bind:this={row} class="select-text">{UserId}</span>
			</Table.Cell>
			<Table.Cell>
				<span bind:this={row} class="select-text">{TimesPlayed}</span>
			</Table.Cell>
			<Table.Cell>
				<DropdownMenu.Root
					onOpenChange={() =>
						setTimeout(
							() => {
								dropdownMenuOpened = !dropdownMenuOpened;
							},
							dropdownMenuOpened ? 100 : 0
						)}
				>
					<DropdownMenu.Trigger asChild let:builder>
						<div class="bg-transparent w-[32px] h-[8px] flex items-center justify-center">
							<Button
								builders={[builder]}
								variant="outline"
								class="size-6 p-0 m-0 transition-opacity delay-150 {!rowSelected &&
								!dropdownMenuOpened
									? 'hidden'
									: ''}"><MoreHorizontal size={12} /></Button
							>
						</div>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-24">
						<DropdownMenu.Item class="cursor-pointer" on:click={openEditDialog}>
							<Edit class="mr-2 size-4" />
							<span>Edit</span>
						</DropdownMenu.Item>
						<DropdownMenu.Item
							class="cursor-pointer dark:bg-red-700 dark:hover:bg-red-600 dark:data-[highlighted]:bg-red-600 bg-red-500 hover:bg-red-400 data-[highlighted]:bg-red-400"
							on:click={deleteSong}
						>
							<X class="mr-2 size-4" />
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
	<ContextMenu.Content>
		<ContextMenu.Item class="cursor-pointer" on:click={openEditDialog}>
			<Edit class="mr-2 size-4" />
			<span>Edit</span>
		</ContextMenu.Item>
		<ContextMenu.Item
			class="cursor-pointer dark:bg-red-700 dark:hover:bg-red-600 dark:data-[highlighted]:bg-red-600 bg-red-500 hover:bg-red-400 data-[highlighted]:bg-red-400"
			on:click={deleteSong}
		>
			<X class="mr-2 size-4" />
			<span>Remove</span></ContextMenu.Item
		>
	</ContextMenu.Content>
</ContextMenu.Root>
