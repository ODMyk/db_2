<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';

	import UsersTab from '$lib/components/users-tab/UsersTab.svelte';
	import SongsTab from '$lib/components/songs-tab/SongsTab.svelte';
	import PlaylistsTab from '$lib/components/playlists-tab/PlaylistsTab.svelte';
	import LinksTab from '$lib/components/links-tab/LinksTab.svelte';
	import TiersTab from '$lib/components/tier-tab/TiersTab.svelte';
	import SqlTab from '$lib/components/sql-tab/SQLTab.svelte';
	import {
		Playlist,
		Link,
		Song,
		Tier,
		User,
		UserSchema,
		TierSchema,
		SongSchema,
		PlaylistSchema,
		LinkSchema
	} from '$lib';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import * as Card from '$lib/components/ui/card';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { ArrowRightFromLine } from 'lucide-svelte';
	import TopBar from '$lib/components/top-bar/TopBar.svelte';
	import { toast } from 'svelte-sonner';

	let loaded = false;
	let failed = false;
	let connectionString = 'host=127.0.0.1 port=5432 dbname=sahodb user=postgres password=postgres';
	let connectionError = '';
	const fetched: {
		tiers: Tier[];
		users: User[];
		songs: Song[];
		playlists: Playlist[];
		links: Link[];
	} = { tiers: [], users: [], songs: [], playlists: [], links: [] };
	const hotkeys = new Map<string, (e: KeyboardEvent) => boolean>();

	const proceed_error = (e: string) => {
		connectionError = e.startsWith('connection to server')
			? 'lost connection to the database'
			: 'chosen database has incorrect schema';
		(document.getElementById('resetButton') as HTMLButtonElement)?.click();
		failed = true;
	};

	const connectionWrapper: <T>(a: () => Promise<T>) => () => Promise<T | undefined> = (a) => {
		return async () => {
			try {
				return await a();
			} catch (e) {
				console.log(e);
				proceed_error(e as string);
			}
		};
	};

	const invalidateTiers = connectionWrapper(async () => {
		const tiers_schema: TierSchema[] = await invoke('get_tiers');
		fetched.tiers.length = 0;
		tiers_schema.forEach((t) => fetched.tiers.push(Tier.fromSchema(t)));
	});

	const invalidateUsers = connectionWrapper(async () => {
		const users_schema: UserSchema[] = await invoke('get_users');
		fetched.users.length = 0;
		users_schema.forEach((u) => fetched.users.push(User.fromSchema(u)));
	});

	const invalidateSongs = connectionWrapper(async () => {
		const songs_schema: SongSchema[] = await invoke('get_songs');
		fetched.songs.length = 0;
		songs_schema.forEach((t) => fetched.songs.push(Song.fromSchema(t)));
	});

	const invalidatePlaylists = connectionWrapper(async () => {
		const playlists_schema: PlaylistSchema[] = await invoke('get_playlists');
		fetched.playlists.length = 0;
		playlists_schema.forEach((t) => fetched.playlists.push(Playlist.fromSchema(t)));
	});

	const invalidateLinks = connectionWrapper(async () => {
		const links_schema: LinkSchema[] = await invoke('get_links');
		fetched.links.length = 0;
		links_schema.forEach((t) => fetched.links.push(Link.fromSchema(t)));
	});

	const invalidate = async () => {
		await invalidateTiers();
		if (failed) {
			return;
		}
		await invalidateUsers();
		if (failed) {
			return;
		}
		await invalidateSongs();
		if (failed) {
			return;
		}
		await invalidatePlaylists();
		if (failed) {
			return;
		}
		await invalidateLinks();
	};

	onMount(async () => {
		window.addEventListener('keydown', (e) => {
			console.log('here');
			const block = settings.hotkey(e);
			if (block) {
				return;
			}

			if (e.ctrlKey && e.keyCode > 48 && e.keyCode < 55) {
				const triggers = document.getElementsByClassName('data-[state=active]:bg-primary');
				const element = triggers[e.keyCode - 49] as HTMLElement;
				element.click();
			}
		});

		const _loaded: boolean = await invoke('init_db');
		failed = false;
		if (_loaded) {
			await invalidate();
			loaded = _loaded;
		}
	});

	const establishConnection = async () => {
		const _loaded: boolean = await invoke('connect_to_db', { connectionString, rewrite: true });
		if (!_loaded) {
			connectionError =
				'unable to establish connection, please check your connection string and try again';
			failed = true;
			return;
		}
		failed = false;
		await invalidate();
		loaded = _loaded && !failed;
		settings.hotkey = () => false;
	};

	const resetConnection = () => {
		failed = false;
		loaded = false;
	};

	const emptyHotkey = (e: KeyboardEvent) => false;

	const settings = { hotkey: (e: KeyboardEvent) => false, callback: () => {} };
</script>

<TopBar Title="Music Admin" {settings} />

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="justify-center flex w-full h-[95vh]"
	on:contextmenu={(event) => {
		event.preventDefault();
	}}
>
	{#if loaded}
		<Button
			class="p-0 size-8 absolute right-10 top-[76px]"
			id="resetButton"
			on:click={resetConnection}
		>
			<ArrowRightFromLine class="size-4" />
		</Button>
		<Tabs.Root
			value="tiers"
			class="w-[960px] select-none"
			onValueChange={(v) => {
				settings.hotkey = hotkeys.get(v?.toString() ?? '') ?? emptyHotkey;
				settings.callback();
			}}
		>
			<div class="flex items-center justify-center my-12">
				<Tabs.List class="grid w-[720px] grid-cols-6 bg-backgroundSecondary">
					<Tabs.Trigger class="data-[state=active]:bg-primary" value="tiers">Tiers</Tabs.Trigger>
					<Tabs.Trigger class="data-[state=active]:bg-primary" value="users">Users</Tabs.Trigger>
					<Tabs.Trigger class="data-[state=active]:bg-primary" value="songs">Songs</Tabs.Trigger>
					<Tabs.Trigger class="data-[state=active]:bg-primary" value="playlists"
						>Playlists</Tabs.Trigger
					>
					<Tabs.Trigger class="data-[state=active]:bg-primary" value="playlistSongs">
						Playlist Songs
					</Tabs.Trigger>
					<Tabs.Trigger class="data-[state=active]:bg-primary" value="sql">SQL</Tabs.Trigger>
				</Tabs.List>
			</div>
			<Tabs.Content value="tiers">
				<TiersTab tiers={fetched.tiers} {hotkeys} {settings} {connectionWrapper} />
			</Tabs.Content>
			<Tabs.Content value="users">
				<UsersTab users={fetched.users} tiers={fetched.tiers} {hotkeys} {connectionWrapper} />
			</Tabs.Content>
			<Tabs.Content value="songs">
				<SongsTab songs={fetched.songs} users={fetched.users} {hotkeys} {connectionWrapper} />
			</Tabs.Content>
			<Tabs.Content value="playlists">
				<PlaylistsTab
					playlists={fetched.playlists}
					users={fetched.users}
					{hotkeys}
					{connectionWrapper}
				/>
			</Tabs.Content>
			<Tabs.Content value="playlistSongs">
				<LinksTab
					songs={fetched.songs}
					playlists={fetched.playlists}
					links={fetched.links}
					{hotkeys}
					{connectionWrapper}
				/>
			</Tabs.Content>
			<Tabs.Content value="sql">
				<SqlTab {hotkeys} {connectionWrapper} {settings} />
			</Tabs.Content>
		</Tabs.Root>
	{:else}
		<div class="flex items-center w-full justify-center">
			<Card.Root class="bg-backgroundSecondary border-none w-2/3">
				<Card.Content class="space-y-3 py-5 items-start flex flex-col">
					<h3 class="select-none cursor-default">
						Set the connection string to your PostgreSQL database
					</h3>
					{#if failed}
						<span class="text-red-600 text-sm">Error: {connectionError}</span>
					{/if}
					<Input
						placeholder="host=127.0.0.1 port=5432 dbname=mydb user=postgres password=postgres"
						bind:value={connectionString}
					/>
					<Button class="place-self-center" on:click={establishConnection}>Connect</Button>
				</Card.Content>
			</Card.Root>
		</div>
	{/if}
</div>
