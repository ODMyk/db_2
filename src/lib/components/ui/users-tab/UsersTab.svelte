<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import UserRow from '$lib/components/ui/user-row/UserRow.svelte';
	import type { Tier, User } from '$lib';

	import Button from '../button/button.svelte';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import { toast } from 'svelte-sonner';

	export let users: User[];
	export let tiers: Tier[];

	let creating = false;

	let newTierId = 0;
	let newNickname: string = '';
	let newEmail: string = '';
	let newPassword: string = '';

	const openCreateDialog = () => {
		if (tiers.length === 0) {
			toast.error("You can't create user because there is no tier to attach");
			return;
		}
		creating = true;
		newTierId = tiers[0].Id;
		newNickname = '';
		newEmail = '';
		newPassword = '';
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

	const createUser = () => {
		toast.success('Successfully created user');
		creating = false;
	};
</script>

<Dialog.Root bind:open={creating} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Create new User</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Tier</div>
				<div class="flex items-center h-12 text-sm">Nickname</div>
				<div class="flex items-center h-12 text-sm">Email</div>
				<div class="flex items-center h-12 text-sm">Password</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<div class="h-12 flex items-center">
					<Select.Root onSelectedChange={(v) => (newTierId = Number.parseInt(`${v?.value}`))}>
						<Select.Trigger class="bg-backgroundSecondary h-12">
							<Select.Value placeholder={tiers[0].Id.toString() + ' | ' + tiers[0].Title} />
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<Select.Group>
								{#each tiers as { Title, Id }}
									<Select.Item value={Id} label={Id.toString() + ' | ' + Title} />
								{/each}
							</Select.Group>
						</Select.Content>
					</Select.Root>
				</div>
				<Input bind:value={newNickname} placeholder="JohnDoe" class="h-12 bg-backgroundSecondary" />
				<Input
					bind:value={newEmail}
					placeholder="johndoe@example.com"
					class="h-12 bg-backgroundSecondary"
				/>
				<Input bind:value={newPassword} class="h-12 bg-backgroundSecondary" />
			</div>
		</div>

		<Dialog.Footer>
			<Button class="bg-primary size-12 p-0" type="submit" on:click={createUser}
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
						<Table.Head class="w-[89px]">TierId</Table.Head>
						<Table.Head class="w-[180px]">Nickname</Table.Head>
						<Table.Head class="w-[280px]">Email</Table.Head>
						<Table.Head class="w-[280px]">Password</Table.Head>
						<Table.Head class="flex items-center justify-center">
							<Button class="size-5 p-0 m-0" on:click={openCreateDialog}
								><Plus class="size-4" /></Button
							>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each users as user}
						<UserRow
							Id={user.Id}
							Nickname={user.Nickname}
							Email={user.Email}
							Password={user.Password}
							TierId={user.Tier}
							{tiers}
						/>
					{/each}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Card.Content>
</Card.Root>
