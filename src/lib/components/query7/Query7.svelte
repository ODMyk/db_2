<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Check, Settings } from 'lucide-svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '$lib/components/ui/label';
	import { Playlist, type PlaylistSchema } from '$lib';
	import { Checkbox } from '../ui/checkbox';
	import { validateInteger, validateUnsigned } from '$lib/utils';
	import * as Card from '$lib/components/ui/card';
	import { invoke } from '@tauri-apps/api';
	import { toast } from 'svelte-sonner';

	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const errors = { ID: '', Count: '' };

	const playlists: Playlist[] = [];

	let updater = false;
	let Count = 5;
	let PlaylistID = 10;

	let editMode = false;
	let newPlaylistID = '';
	let newCount = '';

	const apply = () => {
		errors.Count = '';
		errors.ID = '';
		let id = 0;
		let count = 0;
		const idOk = validateInteger(
			newPlaylistID,
			(n) => (id = n),
			(m) => (errors.ID = m)
		);
		const countOk = validateUnsigned(
			newCount,
			(n) => (count = n),
			(m) => (errors.Count = m)
		);
		if (!idOk || !countOk) {
			return;
		}
		PlaylistID = id;
		Count = count;
		runQuery();
		editMode = false;
		updater = !updater;
	};

	const runQuery = async () => {
		const result = await connectionWrapper(async () => {
			const r: PlaylistSchema[] = await invoke('query7', {
				playlistId: PlaylistID,
				songsCount: Count
			});
			return r;
		})();

		if (result === undefined) {
			toast.error('An error occurred while trying to fetch data from the server');
			return;
		}

		playlists.length = 0;
		result.forEach((el) => playlists.push(Playlist.fromSchema(el)));
		updater = !updater;
	};

	const openConfig = () => {
		newPlaylistID = PlaylistID.toString();
		newCount = Count.toString();
		errors.ID = '';
		errors.Count = '';
		editMode = true;
		setTimeout(() => {
			const overlays = document.getElementsByClassName('backdrop-blur-sm');
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

	runQuery();
</script>

<Dialog.Root bind:open={editMode}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Edit parameters</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex flex-col justify-center text-sm">
					<div class="h-12 flex items-center">Playlist ID</div>
					<div>
						{#if errors.ID}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="h-12 flex items-center">Songs Count</div>
					<div>
						{#if errors.Count}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
			</div>
			<div class="col-span-3 space-y-4">
				<div>
					<Input bind:value={newPlaylistID} class="h-12 bg-backgroundSecondary" />
					{#if errors.ID}
						<div class="mt-1">
							<p class="text-xs text-red-600">
								{errors.ID}
							</p>
						</div>
					{/if}
				</div>
				<div>
					<Input bind:value={newCount} class="h-12 bg-backgroundSecondary" />
					{#if errors.Count}
						<div class="mt-1">
							<p class="text-xs text-red-600">
								{errors.Count}
							</p>
						</div>
					{/if}
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={apply}
				><Check size={24} /></Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<Card.Root class="h-[70vh] bg-backgroundSecondary border-none">
	<Card.Content class="space-y-2 h-full">
		<p class="text-sm text-justify select-text">
			Get all the playlists, that are owned by user who uploaded at least <span
				class="text-primary font-bold">{Count}</span
			>
			songs, and contain every song from playlist with ID
			<span class="text-primary font-bold">{PlaylistID}</span>
		</p>
		<ScrollArea class="h-[94%] w-full" type="scroll" hideDelay={0}>
			<Table.Root class="w-full border-spacing-0 border-separate">
				<Table.Header>
					<Table.Row class="hover:bg-backgroundSecondary">
						<Table.Head class="w-[89px]">Id</Table.Head>
						<Table.Head class="w-[89px]">UserId</Table.Head>
						<Table.Head class="w-[650px]">Title</Table.Head>
						<Table.Head class="w-[90px]">IsPrivate</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button class="size-5 p-0 m-0" on:click={openConfig}
								><Settings class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if updater}
						{#each playlists as playlist}
							<Table.Row class="cursor-default">
								<Table.Cell>
									{playlist.Id}
								</Table.Cell>
								<Table.Cell>
									{playlist.UserId}
								</Table.Cell>
								<Table.Cell>
									{playlist.Title}
								</Table.Cell>
								<Table.Cell class="p-0">
									<div class="flex w-full justify-center">
										<Checkbox
											disabled
											checked={playlist.IsPrivate}
											class="disabled:hover:cursor-default"
										/>
									</div>
								</Table.Cell>
							</Table.Row>
						{/each}
					{:else}
						{#each playlists as playlist}
							<Table.Row class="cursor-default">
								<Table.Cell>
									{playlist.Id}
								</Table.Cell>
								<Table.Cell>
									{playlist.UserId}
								</Table.Cell>
								<Table.Cell>
									{playlist.Title}
								</Table.Cell>
								<Table.Cell class="p-0">
									<div class="flex w-full justify-center">
										<Checkbox
											disabled
											checked={playlist.IsPrivate}
											class="disabled:hover:cursor-default"
										/>
									</div>
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
