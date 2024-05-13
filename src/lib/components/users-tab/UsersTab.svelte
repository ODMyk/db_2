<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import UserRow from '$lib/components/user-row/UserRow.svelte';
	import type { Tier, User } from '$lib';

	import Button from '../ui/button/button.svelte';
	import { Check, Plus } from 'lucide-svelte';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import { validateStringLongerThan, validateStringShorterThan } from '$lib/utils';

	export let users: User[];
	export let tiers: Tier[];
	export let hotkeys: Map<string, (e: KeyboardEvent) => void>;
	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const blocker = { shouldBlock: false };
	const errors = { Nickname: '', Email: '', Password: '' };

	let creating = false;
	let updater = false;

	let newTierId = 0;
	let newNickname: string = '';
	let newEmail: string = '';
	let newPassword: string = '';

	const openCreateDialog = () => {
		if (tiers.length === 0) {
			toast.error("You can't create user because there is no tier to attach");
			return;
		}
		errors.Email = '';
		errors.Nickname = '';
		errors.Password = '';
		creating = true;
		newTierId = tiers[0].Id;
		newNickname = '';
		newEmail = '';
		newPassword = '';
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

	const validateEmail = (text: string, setter: (m: string) => void) => {
		if (!validateText(text, setter)) {
			return false;
		}

		const r = /^\S+@\S+\.\S+$/;
		if (!r.test(text.toLowerCase())) {
			setter('Must be a valid email');
			return false;
		}

		return true;
	};

	const createUser = async () => {
		errors.Email = '';
		errors.Nickname = '';
		errors.Password = '';
		const isNicknameOk = validateText(newNickname, (m) => (errors.Nickname = m));
		const isPasswordOk = validateText(newPassword, (m) => (errors.Password = m));
		const isEmailOk = validateEmail(newEmail, (m) => (errors.Email = m));

		if (!isEmailOk || !isNicknameOk || !isPasswordOk) {
			return;
		}

		const result = await connectionWrapper(async () => {
			const r: number = await invoke('create_user', {
				user: {
					id: 0,
					tier_id: newTierId,
					nickname: newNickname,
					email: newEmail,
					password: newPassword
				}
			});
			return r;
		})();

		if (!result || result < 0) {
			toast.error(`An error occured while trying to create new user`);
			return;
		}
		toast.success('Successfully created user');
		creating = false;
		const i = users.findIndex((u) => u.Id < 0);
		const u: User = {
			Id: result,
			Tier: newTierId,
			Nickname: newNickname,
			Email: newEmail,
			Password: newPassword
		};
		if (i >= 0) {
			users[i] = u;
		} else {
			users.push(u);
		}
		updater = !updater;
	};

	onMount(() => {
		hotkeys.set('users', (e) => {
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
			<Dialog.Title>Create new User</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Tier</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12">Nickname</div>
					<div>
						{#if errors.Nickname}
							<span class="text-red-600 text-xs">Error: </span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12">Email</div>
					<div>
						{#if errors.Email}
							<span class="text-red-600 text-xs">Error: </span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12">Password</div>
					<div>
						{#if errors.Password}
							<span class="text-red-600 text-xs">Error: </span>
						{/if}
					</div>
				</div>
			</div>
			<div class="col-span-3 space-y-4 select-none">
				<div class="h-12 flex items-center">
					<Select.Root onSelectedChange={(v) => (newTierId = Number.parseInt(`${v?.value}`))}>
						<Select.Trigger class="bg-backgroundSecondary h-12">
							<Select.Value placeholder={tiers[0].Id.toString() + ' | ' + tiers[0].Title} />
						</Select.Trigger>
						<Select.Content class="bg-third border-none">
							<ScrollArea class=" h-96" type="scroll" hideDelay={0}>
								<Select.Group>
									{#each tiers as { Title, Id }}
										<Select.Item value={Id} label={Id.toString() + ' | ' + Title} />
									{/each}
								</Select.Group>
							</ScrollArea>
						</Select.Content>
					</Select.Root>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<Input
						bind:value={newNickname}
						placeholder="JohnDoe"
						class="h-12 bg-backgroundSecondary"
					/>
					<div>
						{#if errors.Nickname}
							<span class="text-xs text-red-600">{errors.Nickname}</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<Input
						bind:value={newEmail}
						placeholder="johndoe@example.com"
						class="h-12 bg-backgroundSecondary"
					/>
					<div>
						{#if errors.Email}
							<span class="text-xs text-red-600">{errors.Email}</span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<Input bind:value={newPassword} class="h-12 bg-backgroundSecondary" />
					<div>
						{#if errors.Password}
							<span class="text-xs text-red-600">{errors.Password}</span>
						{/if}
					</div>
				</div>
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
					{#if updater}
						{#each users as user}
							{#if user.Id >= 0}
								<UserRow
									Id={user.Id}
									Nickname={user.Nickname}
									Email={user.Email}
									Password={user.Password}
									TierId={user.Tier}
									{tiers}
									deletef={() => {
										users[users.findIndex((u) => u.Id === user.Id)].Id = -1;
									}}
									edit={(u) => {
										users[users.findIndex((u) => u.Id === user.Id)] = u;
									}}
									{blocker}
									{connectionWrapper}
								/>
							{/if}
						{/each}
					{:else}
						{#each users as user}
							{#if user.Id >= 0}
								<UserRow
									Id={user.Id}
									Nickname={user.Nickname}
									Email={user.Email}
									Password={user.Password}
									TierId={user.Tier}
									{tiers}
									deletef={() => {
										users[users.findIndex((u) => u.Id === user.Id)].Id = -1;
									}}
									edit={(u) => {
										users[users.findIndex((u) => u.Id === user.Id)] = u;
									}}
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
