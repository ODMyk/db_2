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
</script>

<TopBar Title="Incredible Title" />

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="justify-center flex w-full h-[95vh]"
	on:contextmenu={(event) => {
		event.preventDefault();
	}}
>
	<Tabs.Root value="users" class="w-[960px] select-none">
		<div class="flex items-center justify-center my-12">
			<Tabs.List class="grid w-[600px] grid-cols-5 bg-backgroundSecondary">
				<Tabs.Trigger class="data-[state=active]:bg-background" value="users">Users</Tabs.Trigger>
				<Tabs.Trigger class="data-[state=active]:bg-background" value="songs">Songs</Tabs.Trigger>
				<Tabs.Trigger class="data-[state=active]:bg-background" value="playlists"
					>Playlists</Tabs.Trigger
				>
				<Tabs.Trigger class="data-[state=active]:bg-background" value="playlistSongs">
					Playlist Songs
				</Tabs.Trigger>
				<Tabs.Trigger class="data-[state=active]:bg-background" value="sql">SQL</Tabs.Trigger>
			</Tabs.List>
		</div>
		<Tabs.Content value="users">
			<UsersTab />
		</Tabs.Content>
		<Tabs.Content value="songs">
			<SongsTab />
		</Tabs.Content>
		<Tabs.Content value="playlists">
			<PlaylistsTab />
		</Tabs.Content>
		<Tabs.Content value="playlistSongs">
			<PlaylistSongsTab />
		</Tabs.Content>
	</Tabs.Root>
</div>
