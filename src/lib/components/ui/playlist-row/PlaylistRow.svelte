<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import { Button } from '$lib/components/ui/button';
	import { Check, Edit, MoreHorizontal, Trash } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import { Input } from '$lib/components/ui/input';
	import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import * as Select from '$lib/components/ui/select';
	import type { User } from '$lib';
	import * as Tooltip from '$lib/components/ui/tooltip';

	export let Id: number;
	export let Title: string;
	export let UserId: number;
	export let IsPrivate: boolean;
	export let users: User[];

	let newTitle: string;
	let newUserId: number;
	let newIsPrivate: boolean;

	let row: HTMLElement | null | undefined;
	let rowSelected = false;
	let dropdownMenuOpened = false;
	let actualDropdownMenuOpened = false;
	let contextMenuOpened = false;
	let triggerHidden = true;
	let editMode = false;

	const deletePlaylist = async () => {
		const result: { 0: string; 1: boolean } = await invoke('delete_playlist', { id: Id });
		let id: string | number = 0;
		if (result[1]) {
			id = toast.info(result[0]);
		} else {
			id = toast.error(result[0]);
		}
		setTimeout(() => toast.dismiss(id), 1600);
	};

	const editPlaylist = () => {
		// api-call here
		toast.success('Successfully edited playlist');
		editMode = false;
	};

	const openEditDialog = () => {
		editMode = true;
		newTitle = Title;
		newUserId = UserId;
		newIsPrivate = IsPrivate;
		setTimeout(() => {
			const overlays = document.getElementsByClassName('bg-background/80');
			for (const element of overlays) {
				// element.classList.replace('fixed', 'absolute');
				element.classList.replace('inset-0', 'top-30');
				element.classList.replace('bg-background/80', 'bg-background/90');
				element.classList.add('bottom-0', 'left-0', 'right-0', 'w-full', 'h-[95vh]');
				element.addEventListener('click', () => {
					editMode = false;
				});
			}
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

<Dialog.Root bind:open={editMode}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Edit playlist entry</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Id</div>
				<div class="flex items-center h-12 text-sm">UserId</div>
				<div class="flex items-center h-12 text-sm">Title</div>
				<div class="flex items-center h-12 text-sm">IsPrivate</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<Input disabled value={Id} class="h-12 bg-backgroundSecondary" />
				<Select.Root onSelectedChange={(v) => (newUserId = Number.parseInt(`${v?.value}`))}>
					<Select.Trigger class="bg-backgroundSecondary h-12">
						<Select.Value
							placeholder={UserId.toString() +
								' | ' +
								users.filter((u) => u.Id === UserId)[0].Nickname}
						/>
					</Select.Trigger>
					<Select.Content class="bg-third border-none">
						<Select.Group>
							{#each users as { Nickname, Id }}
								<Select.Item value={Id} label={Id.toString() + ' | ' + Nickname} />
							{/each}
						</Select.Group>
					</Select.Content>
				</Select.Root>
				<Input bind:value={newTitle} class="h-12 bg-backgroundSecondary" />
				<div class="h-12 flex items-center">
					<Switch
						bind:checked={newIsPrivate}
						class="data-[state=unchecked]:bg-backgroundSecondary"
					/>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={editPlaylist}
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
				<Tooltip.Root>
					<Tooltip.Trigger class="w-full">
						<span bind:this={row} class="select-text">{UserId}</span>
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>{users.find((u) => u.Id === UserId)?.Nickname}</p>
					</Tooltip.Content>
				</Tooltip.Root>
			</Table.Cell>
			<Table.Cell>
				<span class="select-text">{Title}</span>
			</Table.Cell>
			<Table.Cell class="p-0">
				<div class="flex w-full justify-center">
					<Checkbox disabled checked={IsPrivate} class="disabled:hover:cursor-default" />
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
							on:click={deletePlaylist}
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
			on:click={deletePlaylist}
		>
			<Trash class="mr-2 size-4 stroke-red-600" />
			<span>Remove</span></ContextMenu.Item
		>
	</ContextMenu.Content>
</ContextMenu.Root>
