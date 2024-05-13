<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import TierRow from '$lib/components/tier-row/TierRow.svelte';
	import type { Tier } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import {
		validateStringLongerThan,
		validateStringShorterThan,
		validateUnsignedDouble
	} from '$lib/utils';

	export let tiers: Tier[];
	export let hotkeys: Map<string, (e: KeyboardEvent) => boolean>;
	export let settings: { hotkey: (e: KeyboardEvent) => boolean };
	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const errors = { Price: '', Title: '' };

	let updater = false;
	let creating = false;

	let newTitle = '';
	let newPrice = '';
	let newLimited = false;

	const blocker = { shouldBlock: false };

	const openCreateDialog = () => {
		creating = true;
		newTitle = '';
		newPrice = '';
		newLimited = false;
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

	const hotkey = (e: KeyboardEvent) => {
		if (e.ctrlKey && (e.key === 'n' || e.key === 'т')) {
			openCreateDialog();
		}

		return creating || blocker.shouldBlock;
	};

	const parseTitle = () => {
		return (
			validateStringLongerThan(newTitle, 0, (msg: string) => (errors.Title = msg)) &&
			validateStringShorterThan(newTitle, 51, (msg: string) => (errors.Title = msg))
		);
	};

	const createTier = async () => {
		let price = 0;
		errors.Price = '';
		errors.Title = '';
		const isTitleOk = parseTitle();
		const isPriceOk = validateUnsignedDouble(
			newPrice,
			(n) => (price = n),
			(m) => (errors.Price = m)
		);

		if (!isTitleOk || !isPriceOk) {
			return;
		}

		const result = await connectionWrapper(async () => {
			const r: number = await invoke('create_tier', {
				tier: {
					id: 0,
					title: newTitle,
					price,
					is_limited: newLimited
				}
			});
			return r;
		})();

		if (!result || result < 0) {
			toast.error(`An error occured while trying to create new tier`);
			return;
		}
		const t = { Id: result, Title: newTitle, Price: price, Limited: newLimited };
		const i = tiers.findIndex((t) => t.Id < 0);
		if (i >= 0) {
			tiers[i] = t;
		} else {
			tiers.push(t);
		}
		updater = !updater;
		toast.success('Successfully created tier');
		creating = false;
	};

	onMount(() => {
		hotkeys.set('tiers', hotkey);

		settings.hotkey = hotkey;
	});
</script>

<Dialog.Root bind:open={creating} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Create new Tier</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12">Title</div>
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">Error: </span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col text-sm justify-center">
					<div class="h-12 flex items-center text-sm">Price</div>
					<div>
						{#if errors.Price}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
				<div class="flex items-center h-12 text-sm">Limited</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<div class="flex flex-col justify-center text-sm">
					<Input bind:value={newTitle} placeholder="Extended" class="h-12 bg-backgroundSecondary" />
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">{errors.Title}</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<Input bind:value={newPrice} class="h-12 bg-backgroundSecondary" />

					<div>
						{#if errors.Price}
							<span class="text-xs text-red-600">{errors.Price}</span>
						{/if}
					</div>
				</div>
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
							{#if Id >= 0}
								<TierRow
									{Id}
									{Title}
									{Price}
									{Limited}
									deletef={() => (tiers[tiers.findIndex((t) => t.Id === Id)].Id = -1)}
									edit={(t) => (tiers[tiers.findIndex((t) => t.Id === Id)] = t)}
									{blocker}
									{connectionWrapper}
								/>
							{/if}
						{/each}
					{:else}
						{#each tiers as { Id, Title, Price, Limited }}
							{#if Id >= 0}
								<TierRow
									{Id}
									{Title}
									{Price}
									{Limited}
									deletef={() => (tiers[tiers.findIndex((t) => t.Id === Id)].Id = -1)}
									edit={(t) => (tiers[tiers.findIndex((t) => t.Id === Id)] = t)}
									{blocker}
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
