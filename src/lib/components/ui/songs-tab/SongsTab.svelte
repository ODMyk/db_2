<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import SongRow from '$lib/components/ui/song-row/SongRow.svelte';
	import type { Song, User } from '$lib';
	import Button from '../button/button.svelte';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import { toast } from 'svelte-sonner';

	export let songs: Song[];
	export let users: User[];
	let updater = false;
	let creating = false;

	let newUserId = 0;
	let newTitle = '';
	let newTimesPlayed = 0;

	const openCreateDialog = () => {
		if (users.length === 0) {
			toast.error("You can't create song because there is no user to attach");
			return;
		}
		creating = true;
		newUserId = users[0].Id;
		newTitle = '';
		newTimesPlayed = 0;
		setTimeout(() => {
			const overlays = document.getElementsByClassName('bg-background/80');
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

	const createSong = () => {
		toast.success('Successfully created song');
		creating = false;
	};
</script>

<Dialog.Root bind:open={creating} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Create new Song</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">User</div>
				<div class="flex items-center h-12 text-sm">Title</div>
				<div class="flex items-center h-12 text-sm">Times Played</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<div class="h-12 flex items-center">
					<Select.Root onSelectedChange={(v) => (newUserId = Number.parseInt(`${v?.value}`))}>
						<Select.Trigger class="bg-backgroundSecondary h-12">
							<Select.Value placeholder={users[0].Id.toString() + ' | ' + users[0].Nickname} />
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<Select.Group>
								{#each users as { Nickname, Id }}
									<Select.Item value={Id} label={Id.toString() + ' | ' + Nickname} />
								{/each}
							</Select.Group>
						</Select.Content>
					</Select.Root>
				</div>
				<Input
					bind:value={newTitle}
					placeholder="Smells Like Teen Spirit"
					class="h-12 bg-backgroundSecondary"
				/>
				<Input bind:value={newTimesPlayed} class="h-12 bg-backgroundSecondary" />
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
							<SongRow {Title} {Id} {UserId} {TimesPlayed} {users} />
						{/each}
					{:else}
						{#each songs as { Id, Title, TimesPlayed, UserId }}
							<SongRow {Title} {Id} {UserId} {TimesPlayed} {users} />
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
