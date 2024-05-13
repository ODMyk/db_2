<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as Table from '$lib/components/ui/table';
	import { Song, type SongSchema } from '$lib';
	import * as Card from '$lib/components/ui/card';
	import { invoke } from '@tauri-apps/api';
	import { toast } from 'svelte-sonner';

	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;
	const songs: Song[] = [];

	let updater = false;

	const runQuery = async () => {
		const result = await connectionWrapper(async () => {
			const r: SongSchema[] = await invoke('query8');
			return r;
		})();

		if (result === undefined) {
			toast.error('An error occured while trying to fetch data from the server');
			return;
		}

		songs.length = 0;
		result.forEach((s) => songs.push(Song.fromSchema(s)));
	};

	runQuery();
</script>

<Card.Root class="h-[70vh] bg-backgroundSecondary border-none">
	<Card.Content class="space-y-2 h-full">
		<p class="text-sm text-justify select-text">
			Get all the songs that are included into every playlist
		</p>
		<ScrollArea class="h-[98%] w-full" type="scroll" hideDelay={0}>
			<Table.Root class="w-full border-spacing-0 border-separate">
				<Table.Header>
					<Table.Row class="hover:bg-backgroundSecondary">
						<Table.Head class="w-[120px]">Id</Table.Head>
						<Table.Head class="w-[120px]">TimesPlayed</Table.Head>
						<Table.Head class="w-[710px]">Title</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<div class="size-4 bg-backgroundSecondary" />
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
