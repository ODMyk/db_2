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
	import * as Select from '$lib/components/ui/select';
	import type { Playlist, PlaylistSong, Song } from '$lib';
	import * as Tooltip from '$lib/components/ui/tooltip';

	export let PlaylistId: number;
	export let SongId: number;

	export let playlists: Playlist[];
	export let songs: Song[];
	export let availableLinks: PlaylistSong[];

	let newPlaylistId: number;
	let newSongId: number;

	let row: HTMLElement | null | undefined;
	let rowSelected = false;
	let dropdownMenuOpened = false;
	let actualDropdownMenuOpened = false;
	let contextMenuOpened = false;
	let triggerHidden = true;
	let editMode = false;

	let updater = false;

	const deleteLink = async () => {
		// const result: { 0: string; 1: boolean } = await invoke('delete_user', { id: Id });
		// let id: string | number = 0;
		// if (result[1]) {
		// 	id = toast.info(result[0]);
		// } else {
		// 	id = toast.error(result[0]);
		// }
		// setTimeout(() => toast.dismiss(id), 2600);
	};

	const editLink = () => {
		// api-call here
		toast.success('Successfully edited link');
		editMode = false;
		1;
	};

	const openEditDialog = () => {
		editMode = true;
		newPlaylistId = PlaylistId;
		newSongId = SongId;
		setTimeout(() => {
			const overlays = document.getElementsByClassName('bg-background/80');
			for (const element of overlays) {
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

<Dialog.Root bind:open={editMode} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Edit Playlist-Song link entry</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">PlaylistId</div>
				<div class="flex items-center h-12 text-sm">SongId</div>
			</div>
			<div class="col-span-3 space-y-4">
				<div class="h-12 flex items-center">
					<Select.Root onSelectedChange={(v) => (newPlaylistId = Number.parseInt(`${v?.value}`))}>
						<Select.Trigger class="bg-backgroundSecondary h-12">
							<Select.Value
								placeholder={PlaylistId.toString() +
									' | ' +
									playlists.filter((p) => p.Id === PlaylistId)[0].Title}
							/>
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<Select.Group>
								{#each playlists.filter((p) => (p.Id === PlaylistId && newSongId === SongId) || availableLinks.find((l) => l.PlaylistId === p.Id && l.SongId === newSongId)) as { Title, Id }}
									<Select.Item value={Id} label={Id.toString() + ' | ' + Title} />
								{/each}
							</Select.Group>
						</Select.Content>
					</Select.Root>
				</div>
				<div class="h-12 flex items-center">
					<Select.Root onSelectedChange={(v) => (newSongId = Number.parseInt(`${v?.value}`))}>
						<Select.Trigger class="bg-backgroundSecondary h-12">
							<Select.Value
								placeholder={SongId.toString() +
									' | ' +
									songs.filter((s) => s.Id === SongId)[0].Title}
							/>
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<Select.Group>
								{#each songs.filter((s) => (s.Id === SongId && newPlaylistId === PlaylistId) || availableLinks.find((l) => l.SongId === s.Id && l.PlaylistId === newPlaylistId)) as { Title, Id }}
									<Select.Item value={Id} label={Id.toString() + ' | ' + Title} />
								{/each}
							</Select.Group>
						</Select.Content>
					</Select.Root>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={editLink}
				><Check size={24} /></Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ContextMenu.Root bind:open={contextMenuOpened}>
	<ContextMenu.Trigger asChild let:builder>
		<Table.Row class="cursor-default">
			<Table.Cell>
				<Tooltip.Root>
					<Tooltip.Trigger class="w-full">
						<span bind:this={row} class="select-text">{PlaylistId}</span>
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>{playlists.find((p) => p.Id === PlaylistId)?.Title}</p>
					</Tooltip.Content>
				</Tooltip.Root>
			</Table.Cell>
			<Table.Cell>
				<Tooltip.Root>
					<Tooltip.Trigger class="w-full">
						<span bind:this={row} class="select-text">{SongId}</span>
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>{songs.find((s) => s.Id === SongId)?.Title}</p>
					</Tooltip.Content>
				</Tooltip.Root>
			</Table.Cell>
			<Table.Cell />
			<Table.Cell>
				<DropdownMenu.Root
					bind:open={actualDropdownMenuOpened}
					onOpenChange={() =>
						setTimeout(
							() => {
								dropdownMenuOpened = actualDropdownMenuOpened;
							},
							dropdownMenuOpened ? 100 : 0
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
						class="w-[100px] p-0 bg-background border-third rounded-none"
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
							on:click={deleteLink}
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
			on:click={deleteLink}
		>
			<Trash class="mr-2 size-4 stroke-red-600" />
			<span>Remove</span></ContextMenu.Item
		>
	</ContextMenu.Content>
</ContextMenu.Root>
