<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import SongRow from '$lib/components/ui/song-row/SongRow.svelte';
</script>

<Card.Root class="h-[75vh] bg-backgroundSecondary">
	<Card.Content class="space-y-2">
		<ScrollArea class="h-[75vh] w-full" type="scroll" hideDelay={0}>
			<Table.Root class="w-full border-spacing-0">
				<Table.Header>
					<Table.Row class="hover:bg-backgroundSecondary">
						<Table.Head class="w-[120px]">Id</Table.Head>
						<Table.Head class="w-[568px]">Title</Table.Head>
						<Table.Head class="w-[120px]">UserId</Table.Head>
						<Table.Head class="w-[120px]">Times Played</Table.Head>
						<Table.Head class="w-[32px]"></Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					<SongRow Id={1} Title="Smells Like Teen Spirit" UserId={1} TimesPlayed={12415} />
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
