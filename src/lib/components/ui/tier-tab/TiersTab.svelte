<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import TierRow from '$lib/components/ui/tier-row/TierRow.svelte';
	import type { Tier } from '$lib';
	import Button from '../button/button.svelte';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import { toast } from 'svelte-sonner';

	export let tiers: Tier[];
	let updater = false;
	let creating = false;

	let newTitle = '';
	let newPrice = 0;
	let newLimited = false;

	const openCreateDialog = () => {
		creating = true;
		newTitle = '';
		newPrice = 0;
		newLimited = false;
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

	const createTier = () => {
		toast.success('Successfully created tier');
		creating = false;
	};
</script>

<Dialog.Root bind:open={creating} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Create new Tier</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Title</div>
				<div class="flex items-center h-12 text-sm">Price</div>
				<div class="flex items-center h-12 text-sm">Limited</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<Input bind:value={newTitle} placeholder="Extended" class="h-12 bg-backgroundSecondary" />
				<Input bind:value={newPrice} class="h-12 bg-backgroundSecondary" />
				<div class="h-12 flex items-center">
					<Switch bind:checked={newLimited} class="data-[state=unchecked]:bg-backgroundSecondary" />
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={createTier}
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
						<Table.Head class="w-[120px]">Id</Table.Head>
						<Table.Head class="w-[614px]">Title</Table.Head>
						<Table.Head class="w-[120px]">Price</Table.Head>
						<Table.Head class="w-[64px]">Limited</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button class="size-5 p-0 m-0" on:click={openCreateDialog}
								><Plus class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if updater}
						{#each tiers as { Id, Title, Price, Limited }}
							<TierRow {Id} {Title} {Price} {Limited} />
						{/each}
					{:else}
						{#each tiers as { Id, Title, Price, Limited }}
							<TierRow {Id} {Title} {Price} {Limited} />
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
