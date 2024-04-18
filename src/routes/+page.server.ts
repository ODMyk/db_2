import type { Playlist, PlaylistSong, Song, Tier, User } from "$lib";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = () => {
    const tiers: Tier[] = [
        { Id: 1, Title: 'Base', Price: 0, Limited: false },
        { Id: 2, Title: 'Plus', Price: 100, Limited: false },
        { Id: 3, Title: 'Ultra', Price: 300, Limited: false },
        { Id: 4, Title: 'April-NoReal', Price: 250, Limited: true },
    ];

    const users: User[] = [
        {
            Id: 1,
            Nickname: 'DMykO',
            Email: 'ostapenko.dmytro05@gmail.com',
            Password: 'LAKDJBbgvksdga3#SDGK',
            Tier: 3
        },

        {
            Id: 2,
            Nickname: 'VlaPavlen',
            Email: 'vladyslav.gay@gmail.com',
            Password: 'frontend>>>',
            Tier: 1
        }
    ];

    const songs: Song[] = [
        { Id: 1, UserId: 1, Title: "Smells Like Teen Spirit", TimesPlayed: 12414 },
        { Id: 2, UserId: 1, Title: "Another Love", TimesPlayed: 41245 },
        { Id: 3, UserId: 1, Title: "Tears Come From The Heart", TimesPlayed: 45532 },
    ];

    const playlists: Playlist[] = [
        { Id: 1, UserId: 1, Title: "Genius Playlist", IsPrivate: true },
        { Id: 2, UserId: 2, Title: "Another Playlist", IsPrivate: false },
        { Id: 3, UserId: 1, Title: "Third Playlist", IsPrivate: false },
    ];

    const links: PlaylistSong[] = [
        { PlaylistId: 1, SongId: 1 },
        { PlaylistId: 1, SongId: 2 },
    ];

    return { tiers, users, songs, playlists, links };
}

/* TODO:
 - Validation
 - SQL tab
 - Backend instead of local lists
 - Hotkeys
*/