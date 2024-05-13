<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import type { Playlist, User } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import * as Select from '$lib/components/ui/select';
	import { toast } from 'svelte-sonner';
	import PlaylistRow from '$lib/components/playlist-row/PlaylistRow.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import { validateStringLongerThan, validateStringShorterThan } from '$lib/utils';
	import TierRow from '../tier-row/TierRow.svelte';

	export let playlists: Playlist[];
	export let users: User[];
	export let hotkeys: Map<string, (e: KeyboardEvent) => void>;
	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const errors = { Title: '' };
	const blocker = { shouldBlock: false };
	let updater = false;
	let creating = false;

	let newUserId = 0;
	let newTitle = '';
	let newIsPrivate = false;

	const openCreateDialog = () => {
		errors.Title = '';
		if (users.length === 0) {
			toast.error("You can't create playlist because there is no user to attach");
			return;
		}
		creating = true;
		newUserId = users[0].Id;
		newTitle = '';
		newIsPrivate = false;
		setTimeout(() => {
			const overlays = document.getElementsByClassName('backdrop-blur-sm');
			for (const element of overlays) {
				// element.classList.replace('fixed', 'absolute');
				element.classList.replace('inset-0', 'top-30');
				element.classList.replace('bg-background/80', 'bg-background/90');
				element.classList.add('bottom-0', 'left-0', 'right-0', 'w-full', 'h-[95vh]');
				element.addEventListener('click', () => {
					creating = false;
				});
			}
		}, 1);
	};

	const validateText = (text: string, setter: (m: string) => void) => {
		return validateStringLongerThan(text, 0, setter) && validateStringShorterThan(text, 51, setter);
	};

	const createPlaylist = async () => {
		errors.Title = '';
		if (!validateText(newTitle, (m) => (errors.Title = m))) {
			return;
		}

		const result = await connectionWrapper(async () => {
			const r: number = await invoke('create_playlist', {
				playlist: { id: 0, user_id: newUserId, is_private: newIsPrivate, title: newTitle }
			});
			return r;
		})();

		if (!result || result < 0) {
			toast.error(`An error occured while trying to create playlist`);
			return;
		}

		const p = { Id: result, Title: newTitle, IsPrivate: newIsPrivate, UserId: newUserId };
		const i = playlists.findIndex((p) => p.Id < 0);

		if (i >= 0) {
			playlists[i] = p;
		} else {
			playlists.push(p);
		}

		toast.success('Successfully created playlist');
		creating = false;
		updater = !updater;
	};

	onMount(() => {
		hotkeys.set('playlists', (e) => {
			if (e.ctrlKey && (e.key === 'n' || e.key === 'т')) {
				openCreateDialog();
			}

			return creating || blocker.shouldBlock;
		});
	});
</script>

<Dialog.Root bind:open={creating} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Create new Playlist</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">User</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12 text-sm">Title</div>
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">Error: </span>
						{/if}
					</div>
				</div>
				<div class="flex items-center h-12 text-sm">Is Private</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<div class="h-12 flex items-center">
					<Select.Root onSelectedChange={(v) => (newUserId = Number.parseInt(`${v?.value}`))}>
						<Select.Trigger class="bg-backgroundSecondary h-12">
							<Select.Value placeholder={users[0].Id.toString() + ' | ' + users[0].Nickname} />
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<ScrollArea class=" h-96" type="scroll" hideDelay={0}>
								<Select.Group>
									{#each users as { Nickname, Id }}
										<Select.Item value={Id} label={Id.toString() + ' | ' + Nickname} />
									{/each}
								</Select.Group>
							</ScrollArea>
						</Select.Content>
					</Select.Root>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<Input
						bind:value={newTitle}
						placeholder="Spring Melodies"
						class="h-12 bg-backgroundSecondary"
					/>
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">{errors.Title}</span>
						{/if}
					</div>
				</div>
				<div class="h-12 flex items-center">
					<Switch
						bind:checked={newIsPrivate}
						class="data-[state=unchecked]:bg-backgroundSecondary"
					/>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={createPlaylist}
				><Check size={24} /></Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<Card.Root class="h-[75vh] bg-backgroundSecondary border-none">
	<Card.Content class="space-y-2">
		<ScrollArea class="h-[75vh] w-full" type="scroll" hideDelay={0}>
			<Table.Root class="w-full border-spacing-0 border-separate">
				<Table.Header>
					<Table.Row class="hover:bg-backgroundSecondary">
						<Table.Head class="w-[89px]">Id</Table.Head>
						<Table.Head class="w-[89px]">UserId</Table.Head>
						<Table.Head class="w-[650px]">Title</Table.Head>
						<Table.Head class="w-[90px]">IsPrivate</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button class="size-5 p-0 m-0" on:click={openCreateDialog}
								><Plus class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if updater}
						{#each playlists as { Id, Title, IsPrivate, UserId }}
							{#if Id >= 0}
								<PlaylistRow
									{Title}
									{Id}
									{UserId}
									{IsPrivate}
									{users}
									{blocker}
									edit={(p) => (playlists[playlists.findIndex((p) => p.Id === Id)] = p)}
									deletef={() => (playlists[playlists.findIndex((p) => p.Id === Id)].Id = -1)}
									{connectionWrapper}
								/>
							{/if}
						{/each}
					{:else}
						{#each playlists as { Id, Title, IsPrivate, UserId }}
							{#if Id >= 0}
								<PlaylistRow
									{Title}
									{Id}
									{UserId}
									{IsPrivate}
									{users}
									{blocker}
									edit={(p) => (playlists[playlists.findIndex((p) => p.Id === Id)] = p)}
									deletef={() => (playlists[playlists.findIndex((p) => p.Id === Id)].Id = -1)}
									{connectionWrapper}
								/>
							{/if}
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
