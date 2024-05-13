<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import SongRow from '$lib/components/song-row/SongRow.svelte';
	import type { Song, User } from '$lib';
	import { Button } from '$lib/components/ui/button';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import {
		validateStringLongerThan,
		validateStringShorterThan,
		validateUnsigned
	} from '$lib/utils';

	export let songs: Song[];
	export let users: User[];
	export let hotkeys: Map<string, (e: KeyboardEvent) => boolean>;
	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const errors = { Title: '', TimesPlayed: '' };

	const blocker = { shouldBlock: false };
	let updater = false;
	let creating = false;

	let newUserId = 0;
	let newTitle = '';
	let newTimesPlayed = '';

	const openCreateDialog = () => {
		if (users.length === 0) {
			toast.error("You can't create song because there is no user to attach");
			return;
		}
		errors.TimesPlayed = '';
		errors.Title = '';
		creating = true;
		newUserId = users[0].Id;
		newTitle = '';
		newTimesPlayed = '';
		setTimeout(() => {
			const overlays = document.getElementsByClassName('backdrop-blur-sm');
			for (const element of overlays) {
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

	const createSong = async () => {
		errors.TimesPlayed = '';
		errors.Title = '';
		let times_played = 0;
		const isTitleOk = validateText(newTitle, (m) => (errors.Title = m));
		const isTimesPLayedOk = validateUnsigned(
			newTimesPlayed,
			(n) => (times_played = n),
			(m) => (errors.TimesPlayed = m)
		);

		if (!isTimesPLayedOk || !isTitleOk) {
			return;
		}

		const result = await connectionWrapper(async () => {
			const r: number = await invoke('create_song', {
				song: {
					id: 0,
					user_id: newUserId,
					times_played,
					title: newTitle
				}
			});
			return r;
		})();

		if (!result || result < 0) {
			toast.error(`An error occured while trying to create new song`);
			return;
		}

		toast.success('Successfully created song');
		creating = false;
		const s = { Id: result, UserId: newUserId, TimesPlayed: times_played, Title: newTitle };

		const i = songs.findIndex((s) => s.Id < 0);
		if (i >= 0) {
			songs[i] = s;
		} else {
			songs.push(s);
		}

		updater = !updater;
	};

	onMount(() => {
		hotkeys.set('songs', (e) => {
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
			<Dialog.Title>Create new Song</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">User</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12 text-sm">Title</div>
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12 text-sm">Times Played</div>
					<div>
						{#if errors.TimesPlayed}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
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
						placeholder="Smells Like Teen Spirit"
						class="h-12 bg-backgroundSecondary"
					/>
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">{errors.Title}</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<Input bind:value={newTimesPlayed} class="h-12 bg-backgroundSecondary" />
					<div>
						{#if errors.TimesPlayed}
							<span class="text-xs text-red-600">{errors.TimesPlayed}</span>
						{/if}
					</div>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={createSong}
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
						<Table.Head class="w-[90px]">TimesPlayed</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button class="size-5 p-0 m-0" on:click={openCreateDialog}
								><Plus class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if updater}
						{#each songs as { Id, Title, TimesPlayed, UserId }}
							{#if Id >= 0}
								<SongRow
									{Title}
									{Id}
									{UserId}
									{TimesPlayed}
									{users}
									{blocker}
									deletef={() => (songs[songs.findIndex((s) => s.Id === Id)].Id = -1)}
									edit={() =>
										(songs[songs.findIndex((s) => s.Id === Id)] = {
											Id,
											Title,
											TimesPlayed,
											UserId
										})}
									{connectionWrapper}
								/>
							{/if}
						{/each}
					{:else}
						{#each songs as { Id, Title, TimesPlayed, UserId }}
							{#if Id >= 0}
								<SongRow
									{Title}
									{Id}
									{UserId}
									{TimesPlayed}
									{users}
									{blocker}
									deletef={() => (songs[songs.findIndex((s) => s.Id === Id)].Id = -1)}
									edit={() =>
										(songs[songs.findIndex((s) => s.Id === Id)] = {
											Id,
											Title,
											TimesPlayed,
											UserId
										})}
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
