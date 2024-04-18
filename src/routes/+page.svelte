<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	import TopBar from '$lib/components/ui/top-bar/TopBar.svelte';
	import UsersTab from '$lib/components/ui/users-tab/UsersTab.svelte';
	import SongsTab from '$lib/components/ui/songs-tab/SongsTab.svelte';
	import PlaylistsTab from '$lib/components/ui/playlists-tab/PlaylistsTab.svelte';
	import PlaylistSongsTab from '$lib/components/ui/playlist-songs-tab/PlaylistSongsTab.svelte';
	import TiersTab from '$lib/components/ui/tier-tab/TiersTab.svelte';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import type { Playlist, PlaylistSong, Song, Tier, User } from '$lib';
	import { invalidateAll } from '$app/navigation';

	export let data: PageData;
</script>

<TopBar Title="Incredible Title" />

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="justify-center flex w-full h-[95vh]"
	on:contextmenu={(event) => {
		event.preventDefault();
	}}
>
	<Tabs.Root value="tiers" class="w-[960px] select-none" onValueChange={invalidateAll}>
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
			<TiersTab tiers={data.tiers} />
		</Tabs.Content>
		<Tabs.Content value="users">
			<UsersTab users={data.users} tiers={data.tiers} />
		</Tabs.Content>
		<Tabs.Content value="songs">
			<SongsTab songs={data.songs} users={data.users} />
		</Tabs.Content>
		<Tabs.Content value="playlists">
			<PlaylistsTab playlists={data.playlists} users={data.users} />
		</Tabs.Content>
		<Tabs.Content value="playlistSongs">
			<PlaylistSongsTab songs={data.songs} playlists={data.playlists} links={data.links} />
		</Tabs.Content>
	</Tabs.Root>
</div>
