<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Check, Settings } from 'lucide-svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '$lib/components/ui/label';
	import { Song, SongSchema } from '$lib';
	import { validateInteger, validateUnsigned } from '$lib/utils';
	import { invoke } from '@tauri-apps/api';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import * as Card from '$lib/components/ui/card';

	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const songs: Song[] = [];
	let updater = true;

	const runQuery = async () => {
		const result = await connectionWrapper(async () => {
			const r: SongSchema[] = await invoke('query1', {
				userId: UserID,
				count: Count,
				private: Private,
				public: Public
			});
			return r;
		})();

		if (result === undefined) {
			toast.error('An error occured while trying to fetch data from the server');
			return;
		}

		songs.length = 0;
		result.forEach((s) => songs.push(Song.fromSchema(s)));
	};

	const errors = { ID: '', Count: '' };

	let Count = 5;
	let Private = true;
	let Public = true;
	let UserID = 10;

	let editMode = false;
	let newUserID = '';
	let newCount = '';
	let newStatus = '';

	const apply = async () => {
		errors.Count = '';
		errors.ID = '';
		let id = 0;
		let count = 0;
		const idOk = validateInteger(
			newUserID,
			(n) => (id = n),
			(m) => (errors.ID = m)
		);
		const countOk = validateUnsigned(
			newCount,
			(n) => (count = n),
			(m) => (errors.Count = m)
		);
		if (idOk && countOk) {
			UserID = id;
			Count = count;
			checkStatus();
			await runQuery();
			editMode = false;
		}
	};

	const checkStatus = () => {
		if (newStatus == 'both') {
			Private = true;
			Public = true;
			return;
		}
		Private = newStatus == 'private';
		Public = !Private;
	};
	const openConfig = () => {
		newUserID = UserID.toString();
		newCount = Count.toString();
		newStatus = Private && Public ? 'both' : Private ? 'private' : 'public';
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

	onMount(async () => {
		await runQuery();
	});
</script>

<Dialog.Root bind:open={editMode}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Edit parameters</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex flex-col justify-center text-sm">
					<div class="h-12 flex items-center">UserID</div>
					<div>
						{#if errors.ID}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="h-12 flex items-center">Plays Count</div>
					<div>
						{#if errors.Count}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex items-center h-12 text-sm">Status</div>
			</div>
			<div class="col-span-3 space-y-4">
				<div>
					<Input bind:value={newUserID} class="h-12 bg-backgroundSecondary" />
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
				<RadioGroup.Root bind:value={newStatus}>
					<div class="flex space-x-2">
						<RadioGroup.Item value="public" />
						<Label>Public</Label>
					</div>
					<div class="flex space-x-2">
						<RadioGroup.Item value="private" />
						<Label>Private</Label>
					</div>
					<div class="flex space-x-2">
						<RadioGroup.Item value="both" />
						<Label>Any</Label>
					</div>
				</RadioGroup.Root>
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
			Get all the songs, uploaded by user with ID <span class="text-primary font-bold"
				>{UserID}</span
			>
			with at least <span class="text-primary font-bold">{Count}</span> plays and included in
			<span class="text-primary font-bold"
				>{Private && Public ? 'any' : Private ? 'private' : 'public'}</span
			> playlist
		</p>
		<ScrollArea class="h-[98%] w-full" type="scroll" hideDelay={0}>
			<Table.Root class="w-full border-spacing-0 border-separate">
				<Table.Header>
					<Table.Row class="hover:bg-backgroundSecondary">
						<Table.Head class="w-[120px]">Id</Table.Head>
						<Table.Head class="w-[120px]">TimesPlayed</Table.Head>
						<Table.Head class="w-[710px]">Title</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button on:click={openConfig} class="size-5 p-0 m-0"
								><Settings class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if updater}
						{#each songs as song}
							<Table.Row>
								<Table.Cell>
									{song.Id}
								</Table.Cell>
								<Table.Cell>
									{song.TimesPlayed}
								</Table.Cell>
								<Table.Cell>
									{song.Title}
								</Table.Cell>
							</Table.Row>
						{/each}
					{:else}
						{#each songs as song}
							<Table.Row>
								<Table.Cell>
									{song.Id}
								</Table.Cell>
								<Table.Cell>
									{song.TimesPlayed}
								</Table.Cell>
								<Table.Cell>
									{song.Title}
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
