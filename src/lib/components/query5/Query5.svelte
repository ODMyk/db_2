<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Check, Settings } from 'lucide-svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '$lib/components/ui/label';
	import { Tier, type TierSchema } from '$lib';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { validateUnsigned } from '$lib/utils';
	import * as Card from '$lib/components/ui/card';
	import { toast } from 'svelte-sonner';
	import { invoke } from '@tauri-apps/api';

	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const errors = { UsersCount: '', PlaylistsCount: '' };

	const tiers: Tier[] = [];

	let updater = false;
	let PlaylistsCount = 5;
	let Private = true;
	let Public = true;
	let UsersCount = 10;

	let editMode = false;
	let newUsersCount = '';
	let newPlaylistsCount = '';
	let newStatus = '';

	const apply = () => {
		errors.PlaylistsCount = '';
		errors.UsersCount = '';
		let usersCount = 0;
		let playlistsCount = 0;
		const usersCountOk = validateUnsigned(
			newUsersCount,
			(n) => (usersCount = n),
			(m) => (errors.UsersCount = m)
		);
		const playlistsCountOk = validateUnsigned(
			newPlaylistsCount,
			(n) => (playlistsCount = n),
			(m) => (errors.PlaylistsCount = m)
		);
		if (!usersCountOk || !playlistsCountOk) {
			return;
		}
		UsersCount = usersCount;
		PlaylistsCount = playlistsCount;
		checkStatus();
		runQuery();
		editMode = false;
		updater = !updater;
	};

	const runQuery = async () => {
		const result = await connectionWrapper(async () => {
			const r: TierSchema[] = await invoke('query5', {
				playlistsCount: PlaylistsCount,
				usersCount: UsersCount,
				private: Private,
				public: Public
			});
			return r;
		})();
		if (result === undefined) {
			toast.error('An error occurred while trying to fetch data from the server');
			return;
		}

		tiers.length = 0;
		result.forEach((el) => tiers.push(Tier.fromSchema(el)));
		updater = !updater;
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
		newUsersCount = UsersCount.toString();
		newPlaylistsCount = PlaylistsCount.toString();
		newStatus = Private && Public ? 'both' : Private ? 'private' : 'public';
		errors.UsersCount = '';
		errors.PlaylistsCount = '';
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
					<div class="h-12 flex items-center">Users Count</div>
					<div>
						{#if errors.UsersCount}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="h-12 flex items-center">Playlists Count</div>
					<div>
						{#if errors.PlaylistsCount}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex items-center h-12 text-sm">Status</div>
			</div>
			<div class="col-span-3 space-y-4">
				<div>
					<Input bind:value={newUsersCount} class="h-12 bg-backgroundSecondary" />
					{#if errors.UsersCount}
						<div class="mt-1">
							<p class="text-xs text-red-600">
								{errors.UsersCount}
							</p>
						</div>
					{/if}
				</div>
				<div>
					<Input bind:value={newPlaylistsCount} class="h-12 bg-backgroundSecondary" />
					{#if errors.PlaylistsCount}
						<div class="mt-1">
							<p class="text-xs text-red-600">
								{errors.PlaylistsCount}
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
			Get all the tiers, that are attached to at least <span class="text-primary font-bold"
				>{UsersCount}</span
			>
			users who owns at least <span class="text-primary font-bold">{PlaylistsCount}</span>
			<span class="text-primary font-bold"
				>{Private && Public ? 'any' : Private ? 'private' : 'public'}</span
			> playlists
		</p>
		<ScrollArea class="h-[98%] w-full" type="scroll" hideDelay={0}>
			<Table.Root class="w-full border-spacing-0 border-separate">
				<Table.Header>
					<Table.Row class="hover:bg-backgroundSecondary">
						<Table.Head class="w-[120px]">Id</Table.Head>
						<Table.Head class="w-[614px]">Title</Table.Head>
						<Table.Head class="w-[120px]">Price</Table.Head>
						<Table.Head class="w-[64px]">Limited</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button class="size-5 p-0 m-0" on:click={openConfig}
								><Settings class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if updater}
						{#each tiers as tier}
							<Table.Row>
								<Table.Cell>
									{tier.Id}
								</Table.Cell>
								<Table.Cell>
									{tier.Title}
								</Table.Cell>
								<Table.Cell>
									{tier.Price}
								</Table.Cell>
								<Table.Cell>
									<div class="flex w-full justify-center">
										<Checkbox
											disabled
											checked={tier.Limited}
											class="disabled:hover:cursor-default"
										/>
									</div>
								</Table.Cell>
							</Table.Row>
						{/each}
					{:else}
						{#each tiers as tier}
							<Table.Row>
								<Table.Cell>
									{tier.Id}
								</Table.Cell>
								<Table.Cell>
									{tier.Title}
								</Table.Cell>
								<Table.Cell>
									{tier.Price}
								</Table.Cell>
								<Table.Cell>
									<div class="flex w-full justify-center">
										<Checkbox
											disabled
											checked={tier.Limited}
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
