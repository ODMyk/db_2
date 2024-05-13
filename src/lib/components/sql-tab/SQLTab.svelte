<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Tabs from '$lib/components/ui/tabs';
	import { onMount } from 'svelte';
	import Query1 from '../query1/Query1.svelte';
	import Query2 from '../query2/Query2.svelte';
	import Query3 from '../query3/Query3.svelte';
	import Query4 from '../query4/Query4.svelte';
	import Query5 from '../query5/Query5.svelte';
	import Query6 from '../query6/Query6.svelte';
	import Query7 from '../query7/Query7.svelte';
	import Query8 from '../query8/Query8.svelte';
	import type { Playlist, Song, Tier, User } from '$lib';
	import { invoke } from '@tauri-apps/api';
	import { toast } from 'svelte-sonner';

	export let settings: { callback: () => void };
	export let hotkeys: Map<string, (e: KeyboardEvent) => void>;
	export let connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined>;

	const q1: Song[] = [];
	const q2: Song[] = [];
	const q3: User[] = [];
	const q4: User[] = [];
	const q5: Tier[] = [];
	const q6: Playlist[] = [];
	const q7: Playlist[] = [];
	const q8: Song[] = [];

	const r1 = async (UserId: number, Count: number, Private: boolean, Public: boolean) => {
		const result = await connectionWrapper(async () => {
			const r: Song[] = await invoke('query1', {
				user_id: UserId,
				count: Count,
				private: Private,
				public: Public
			});
			return r;
		})();

		if (!result) {
			toast.error('An error occured while trying to fetch data from the server');
			return;
		}

		q1.length = 0;
		result.forEach((s) => q1.push(s));
		f1 = !f1;
	};

	let f1 = true;
	let f2 = true;
	let f3 = true;
	let f4 = true;
	let f5 = true;
	let f6 = true;
	let f7 = true;
	let f8 = true;

	onMount(() => {
		hotkeys.set('sql', (e) => {
			if (e.altKey && e.keyCode > 48 && e.keyCode < 57) {
				const triggers = document.getElementsByClassName('data-[state=active]:bg-primary');
				const element = triggers[e.keyCode - 43] as HTMLElement;
				element.click();
			}

			return false;
		});

		settings.callback = () => (f1 = !f1);
	});

	const refresh = (v: string | undefined) => {
		if (!v) {
			return;
		}

		switch (v) {
			case '1':
				settings.callback = () => (f1 = !f1);
				break;
			case '2':
				settings.callback = () => (f2 = !f2);
				break;
			case '3':
				settings.callback = () => (f3 = !f3);
				break;
			case '4':
				settings.callback = () => (f4 = !f4);
				break;
			case '5':
				settings.callback = () => (f5 = !f5);
				break;
			case '6':
				settings.callback = () => (f6 = !f6);
				break;
			case '7':
				settings.callback = () => (f7 = !f7);
				break;
			case '8':
				settings.callback = () => (f8 = !f8);
				break;
		}
		settings.callback();
	};
</script>

<Tabs.Root
	value="1"
	class="w-full select-none bg-backgroundSecondary rounded-lg"
	onValueChange={refresh}
>
	<div class="w-full h-full grid grid-cols-12 justify-start">
		<Tabs.List
			class="flex w-[40px] flex-col bg-background self-center items-center justify-center h-max ml-8"
		>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="1">1</Tabs.Trigger>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="2">2</Tabs.Trigger>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="3">3</Tabs.Trigger>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="4">4</Tabs.Trigger>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="5">5</Tabs.Trigger>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="6">6</Tabs.Trigger>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="7">7</Tabs.Trigger>
			<Tabs.Trigger class="data-[state=active]:bg-primary" value="8">8</Tabs.Trigger>
		</Tabs.List>
		<div class="col-span-11 mt-5 h-full">
			<Tabs.Content value="1" class="h-full w-full">
				{#if f1}
					<Query1 {connectionWrapper} />
				{:else}
					<Query1 {connectionWrapper} />
				{/if}
			</Tabs.Content>
			<Tabs.Content value="2" class="h-full w-full">
				{#if f2}
					<Query2 {connectionWrapper} />
				{:else}
					<Query2 {connectionWrapper} />
				{/if}
			</Tabs.Content>
			<Tabs.Content value="3" class="h-full w-full">
				{#if f3}
					<Query3 {connectionWrapper} />
				{:else}
					<Query3 {connectionWrapper} />
				{/if}
			</Tabs.Content>
			<Tabs.Content value="4" class="h-full w-full">
				{#if f4}
					<Query4 {connectionWrapper} />
				{:else}
					<Query4 {connectionWrapper} />
				{/if}
			</Tabs.Content>
			<Tabs.Content value="5" class="h-full w-full">
				{#if f5}
					<Query5 {connectionWrapper} />
				{:else}
					<Query5 {connectionWrapper} />
				{/if}
			</Tabs.Content>
			<Tabs.Content value="6" class="h-full w-full">
				{#if f6}
					<Query6 {connectionWrapper} />
				{:else}
					<Query6 {connectionWrapper} />
				{/if}
			</Tabs.Content>
			<Tabs.Content value="7" class="h-full w-full">
				{#if f7}
					<Query7 {connectionWrapper} />
				{:else}
					<Query7 {connectionWrapper} />
				{/if}
			</Tabs.Content>
			<Tabs.Content value="8" class="h-full w-full">
				{#if f8}
					<Query8 {connectionWrapper} />
				{:else}
					<Query8 {connectionWrapper} />
				{/if}
			</Tabs.Content>
		</div>
	</div>
</Tabs.Root>
