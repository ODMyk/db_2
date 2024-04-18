<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import type { Song, Playlist, PlaylistSong } from '$lib';
	import Button from '../button/button.svelte';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import { toast } from 'svelte-sonner';
	import PlaylistSongRow from '../playlist-song-row/PlaylistSongRow.svelte';

	export let songs: Song[];
	export let playlists: Playlist[];
	export let links: PlaylistSong[];
	const availableLinks: PlaylistSong[] = [];

	let updater = false;
	let creating = false;

	let newPlaylistId = 0;
	let newSongId = 0;

	const openCreateDialog = () => {
		if (playlists.length === 0) {
			toast.error("You can't create the link because there is no playlist to attach");
			return;
		}

		if (songs.length === 0) {
			toast.error("You can't create the link because there is no song to attach");
			return;
		}

		if (links.length === songs.length * playlists.length) {
			// Multiplication rule: A objects of first type, B objects of second type, then number of all possible combinations are A*B
			toast.error("You can't create the link because there are already all possible links");
			return;
		}

		availableLinks.length = 0;
		for (const p of playlists) {
			for (const s of songs) {
				if (!links.find((l) => l.PlaylistId === p.Id && l.SongId === s.Id)) {
					availableLinks.push({ PlaylistId: p.Id, SongId: s.Id });
				}
			}
		}

		creating = true;
		newPlaylistId = playlists[0].Id;
		newSongId = songs[0].Id;
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

	const createLink = () => {
		toast.success('Successfully created link');
		creating = false;
	};
</script>

<Dialog.Root bind:open={creating} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Create new Playlist-Song link</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Playlist</div>
				<div class="flex items-center h-12 text-sm">Song</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<div class="h-12 flex items-center">
					<Select.Root onSelectedChange={(v) => (newPlaylistId = Number.parseInt(`${v?.value}`))}>
						<Select.Trigger class="bg-backgroundSecondary h-12">
							<Select.Value
								placeholder={availableLinks[0].PlaylistId.toString() +
									' | ' +
									playlists.find((p) => p.Id === availableLinks[0].PlaylistId)?.Title}
							/>
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<Select.Group>
								{#each playlists.filter( (p) => availableLinks.find((l) => l.PlaylistId === p.Id && l.SongId === newSongId) ) as { Title, Id }}
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
								placeholder={availableLinks[0].SongId.toString() +
									' | ' +
									songs.find((s) => s.Id === availableLinks[0].SongId)?.Title}
							/>
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<Select.Group>
								{#each songs.filter( (s) => availableLinks.find((l) => l.SongId === s.Id && l.PlaylistId === newPlaylistId) ) as { Title, Id }}
									<Select.Item value={Id} label={Id.toString() + ' | ' + Title} />
								{/each}
							</Select.Group>
						</Select.Content>
					</Select.Root>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={createLink}
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
						<Table.Head class="w-[89px]">PlaylistId</Table.Head>
						<Table.Head class="w-[89px]">SongId</Table.Head>
						<Table.Head class="w-[740px]"></Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button class="size-5 p-0 m-0" on:click={openCreateDialog}
								><Plus class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if updater}
						{#each links as { PlaylistId, SongId }}
							<PlaylistSongRow {PlaylistId} {SongId} {songs} {playlists} {availableLinks} />
						{/each}
					{:else}
						{#each links as { PlaylistId, SongId }}
							<PlaylistSongRow {PlaylistId} {SongId} {songs} {playlists} {availableLinks} />
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
