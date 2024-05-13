<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import { Button } from '$lib/components/ui/button';
	import { Check, Edit, MoreHorizontal, Trash, X } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import type { Song, User } from '$lib';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import {
		validateStringLongerThan,
		validateStringShorterThan,
		validateUnsigned
	} from '$lib/utils';
	import { ScrollArea } from '../ui/scroll-area';

	export let Id: number;
	export let UserId: number;
	export let Title: string;
	export let TimesPlayed: number;
	export let users: User[];
	export let blocker: { shouldBlock: boolean };
	export let deletef: () => void;
	export let edit: (s: Song) => void;
	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const errors = { Title: '', TimesPlayed: '' };

	let newTitle: string;
	let newUserId: number;
	let newTimesPlayed: string;

	let row: HTMLElement | null | undefined;
	let rowSelected = false;
	let dropdownMenuOpened = false;
	let actualDropdownMenuOpened = false;
	let contextMenuOpened = false;
	let triggerHidden = true;
	let editMode = false;

	const deleteSong = async () => {
		const result = await connectionWrapper(async () => {
			const r: boolean = await invoke('delete_song', { id: Id });
			return r;
		})();
		if (!result) {
			toast.error(`An error occured while trying to remove song with Id ${Id}`);
			return;
		}

		toast.success(`Successfully removed song with Id ${Id}`);
		deletef();
	};

	const validateText = (text: string, setter: (m: string) => void) => {
		return validateStringLongerThan(text, 0, setter) && validateStringShorterThan(text, 51, setter);
	};

	const editSong = async () => {
		errors.TimesPlayed = '';
		errors.Title = '';
		let times_played = 0;
		const isTitleOk = validateText(newTitle, (m) => (errors.Title = m));
		const isTimesPlayedOk = validateUnsigned(
			newTimesPlayed,
			(n) => (times_played = n),
			(m) => (errors.TimesPlayed = m)
		);

		if (!isTimesPlayedOk || !isTitleOk) {
			return;
		}

		const result = await connectionWrapper(async () => {
			const r: boolean = await invoke('edit_song', {
				song: {
					id: Id,
					user_id: newUserId,
					times_played,
					title: newTitle
				}
			});
			return r;
		})();

		if (!result) {
			toast.error(`An error occured while trying to edit song with Id ${Id}`);
			return;
		}

		toast.success('Successfully edited song');
		editMode = false;
		Title = newTitle;
		TimesPlayed = times_played;
		UserId = newUserId;
		blocker.shouldBlock = false;
		edit({ Title, TimesPlayed, UserId, Id });
	};

	const openEditDialog = () => {
		errors.TimesPlayed = '';
		errors.Title = '';
		blocker.shouldBlock = true;
		editMode = true;
		newTitle = Title;
		newUserId = UserId;
		newTimesPlayed = TimesPlayed.toString();
		setTimeout(() => {
			const overlays = document.getElementsByClassName('backdrop-blur-sm');
			for (const element of overlays) {
				element.classList.replace('inset-0', 'top-30');
				element.classList.replace('bg-background/80', 'bg-background/90');
				element.classList.add('bottom-0', 'left-0', 'right-0', 'w-full', 'h-[95vh]');
				element.addEventListener('click', () => {
					editMode = false;
					blocker.shouldBlock = false;
				});
			}

			const but = document.getElementsByClassName(
				'absolute right-4 top-4 rounded-sm opacity-70'
			)[0];
			but.addEventListener('click', () => {
				editMode = false;
				blocker.shouldBlock = false;
			});
		}, 1);
	};

	onMount(() => {
		row = row?.parentElement?.parentElement;

		row?.addEventListener('mouseenter', () => {
			rowSelected = true;
		});

		row?.addEventListener('mouseleave', () => {
			rowSelected = false;
		});

		window.addEventListener('mousedown', showTrigger);
		window.addEventListener('mouseup', hideTrigger);

		window.addEventListener('keydown', (e) => {
			if (e.keyCode === 27) {
				blocker.shouldBlock = false;
			}
		});
	});

	const showTrigger = (event: MouseEvent) => {
		if (event.button === 2) {
			contextMenuOpened = false;
			triggerHidden = false;
			actualDropdownMenuOpened = false;
			setTimeout(() => (dropdownMenuOpened = actualDropdownMenuOpened), 100);
		}
	};

	const hideTrigger = (event: MouseEvent) => {
		if (event.button === 2) {
			triggerHidden = true;
		}
	};
</script>

<Dialog.Root bind:open={editMode} closeOnOutsideClick={false}>
	<Dialog.Content class="border-none">
		<Dialog.Header>
			<Dialog.Title>Edit song entry</Dialog.Title>
		</Dialog.Header>

		<div class="grid grid-cols-4 w-full gap-4">
			<div class="col-span-1 space-y-4">
				<div class="flex items-center h-12 text-sm">Id</div>
				<div class="flex items-center h-12 text-sm">User</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12 text-sm">Title</div>
					<div>
						{#if errors.Title}
							<span class="text-xs text-red-600">Error: </span>
						{/if}
					</div>
				</div>
				<div class="flex flex-col justify-center text-sm">
					<div class="flex items-center h-12 text-sm">Times Played</div>
					<div>
						{#if errors.TimesPlayed}
							<span class="text-xs text-red-600">Error: </span>
						{/if}
					</div>
				</div>
			</div>
			<div class="col-span-3 space-y-4">
				<Input disabled value={Id} class="h-12 bg-backgroundSecondary" />
				<Select.Root onSelectedChange={(v) => (newUserId = Number.parseInt(`${v?.value}`))}>
					<Select.Trigger class="bg-backgroundSecondary h-12">
						<Select.Value
							placeholder={UserId.toString() +
								' | ' +
								users.filter((u) => u.Id === UserId)[0].Nickname}
						/>
					</Select.Trigger>
					<Select.Content class="bg-third border-none">
						<ScrollArea class=" h-80" type="scroll" hideDelay={0}>
							<Select.Group>
								{#each users as { Nickname, Id }}
									<Select.Item value={Id} label={Id.toString() + ' | ' + Nickname} />
								{/each}
							</Select.Group>
						</ScrollArea>
					</Select.Content>
				</Select.Root>
				<div class="flex flex-col justify-center text-sm">
					<Input bind:value={newTitle} class="h-12 bg-backgroundSecondary" />
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
			<Button class="bg-primary size-12 p-0" type="submit" on:click={editSong}
				><Check size={24} /></Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ContextMenu.Root bind:open={contextMenuOpened}>
	<ContextMenu.Trigger asChild let:builder>
		<Table.Row class="cursor-default">
			<Table.Cell>
				<span bind:this={row} class="select-text">{Id}</span>
			</Table.Cell>
			<Table.Cell>
				<Tooltip.Root>
					<Tooltip.Trigger class="w-full">
						<span class="select-text">{UserId}</span>
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>{users.find((u) => u.Id === UserId)?.Nickname}</p>
					</Tooltip.Content>
				</Tooltip.Root>
			</Table.Cell>
			<Table.Cell>
				<span class="select-text">{Title}</span>
			</Table.Cell>
			<Table.Cell>
				<span class="select-text">{TimesPlayed}</span>
			</Table.Cell>
			<Table.Cell>
				<DropdownMenu.Root
					bind:open={actualDropdownMenuOpened}
					onOpenChange={() =>
						setTimeout(
							() => {
								dropdownMenuOpened = actualDropdownMenuOpened;
							},
							dropdownMenuOpened ? 100 : 0
						)}
				>
					<DropdownMenu.Trigger asChild let:builder>
						<div class="bg-transparent w-[32px] h-[8px] flex items-center justify-center">
							<Button
								builders={[builder]}
								class="size-5 p-0 m-0 transition-opacity delay-150 {!rowSelected &&
								!dropdownMenuOpened
									? 'hidden'
									: ''}"><MoreHorizontal size={12} /></Button
							>
						</div>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content
						style="min-width: 0;"
						class="w-[100px] p-0 bg-background border-third rounded-none"
					>
						<DropdownMenu.Item
							class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
							on:click={openEditDialog}
						>
							<Edit class="mr-2 size-4" />
							<span>Edit</span>
						</DropdownMenu.Item>
						<DropdownMenu.Item
							class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
							on:click={deleteSong}
						>
							<Trash class="mr-2 size-4 stroke-red-600" />
							<span>Remove</span>
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</Table.Cell>

			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				use:builder.action
				{...builder}
				class="absolute left-0 w-full h-1/3 {triggerHidden ? 'hidden' : ''}"
			/>
		</Table.Row>
	</ContextMenu.Trigger>
	<ContextMenu.Content
		style="min-width: 0;"
		class=" w-[100px] p-0 border-third rounded-none bg-background"
	>
		<ContextMenu.Item
			class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
			on:click={openEditDialog}
		>
			<Edit class="mr-2 size-4" />
			<span>Edit</span>
		</ContextMenu.Item>
		<ContextMenu.Item
			class="cursor-pointer data-[highlighted]:bg-primary rounded-none"
			on:click={deleteSong}
		>
			<Trash class="mr-2 size-4 stroke-red-600" />
			<span>Remove</span></ContextMenu.Item
		>
	</ContextMenu.Content>
</ContextMenu.Root>
