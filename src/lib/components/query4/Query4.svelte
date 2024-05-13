<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Check, Settings } from 'lucide-svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '$lib/components/ui/label';
	import { User, UserSchema } from '$lib';
	import { validateUnsigned } from '$lib/utils';
	import * as Card from '$lib/components/ui/card';
	import { invoke } from '@tauri-apps/api';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';

	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const errors = { Count: '' };

	const users: User[] = [];

	let updater = false;
	let Count = 5;
	let TierTitle = 'Basic';

	let editMode = false;
	let newTierTitle = '';
	let newCount = '';

	const apply = () => {
		errors.Count = '';
		let count = 0;
		const countOk = validateUnsigned(
			newCount,
			(n) => (count = n),
			(m) => (errors.Count = m)
		);
		if (!countOk) {
			return;
		}
		TierTitle = newTierTitle;
		Count = count;
		runQuery();
		editMode = false;
		updater = !updater;
	};

	const runQuery = async () => {
		const result = await connectionWrapper(async () => {
			const r: UserSchema[] = await invoke('query4', { count: Count, title: TierTitle });
			return r;
		})();
		if (result === undefined) {
			toast.error('An error occurred while trying to fetch data from the server');
			return;
		}

		users.length = 0;
		result.forEach((el) => users.push(User.fromSchema(el)));
		updater = !updater;
	};

	const openConfig = () => {
		newTierTitle = TierTitle.toString();
		newCount = Count.toString();
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

	onMount(async () => await runQuery());
</script>

<Dialog.Root bind:open={editMode}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Edit parameters</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex flex-col justify-center text-sm">
					<div class="h-12 flex items-center">Tier Title</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="h-12 flex items-center">Songs Count</div>
					<div>
						{#if errors.Count}
							<span class="text-xs text-red-600">Error:</span>
						{/if}
					</div>
				</div>
			</div>
			<div class="col-span-3 space-y-4">
				<div>
					<Input bind:value={newTierTitle} class="h-12 bg-backgroundSecondary" />
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
			Get all the users, who has account tier with title <span class="text-primary font-bold"
				>{TierTitle}</span
			>
			and owns a playlist with at least <span class="text-primary font-bold">{Count}</span> songs
		</p>
		<ScrollArea class="h-[98%] w-full" type="scroll" hideDelay={0}>
			<Table.Root class="w-full border-spacing-0 border-separate">
				<Table.Root class="w-full border-spacing-0 border-separate">
					<Table.Header>
						<Table.Row class="hover:bg-backgroundSecondary">
							<Table.Head class="w-[89px]">Id</Table.Head>
							<Table.Head class="w-[89px]">TierId</Table.Head>
							<Table.Head class="w-[180px]">Nickname</Table.Head>
							<Table.Head class="w-[280px]">Email</Table.Head>
							<Table.Head class="w-[280px]">Password</Table.Head>
							<Table.Head class="flex items-center justify-center">
								<Button class="size-5 p-0 m-0" on:click={openConfig}
									><Settings class="size-4" /></Button
								>
							</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#if updater}
							{#each users as user}
								<Table.Row>
									<Table.Cell>
										{user.Id}
									</Table.Cell>
									<Table.Cell>
										{user.Tier}
									</Table.Cell>
									<Table.Cell>
										{user.Nickname}
									</Table.Cell>
									<Table.Cell>
										{user.Email}
									</Table.Cell>
									<Table.Cell>
										{user.Password}
									</Table.Cell>
								</Table.Row>
							{/each}
						{:else}
							{#each users as user}
								<Table.Row>
									<Table.Cell>
										{user.Id}
									</Table.Cell>
									<Table.Cell>
										{user.Tier}
									</Table.Cell>
									<Table.Cell>
										{user.Nickname}
									</Table.Cell>
									<Table.Cell>
										{user.Email}
									</Table.Cell>
									<Table.Cell>
										{user.Password}
									</Table.Cell>
								</Table.Row>
							{/each}
						{/if}
					</Table.Body>
				</Table.Root>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
