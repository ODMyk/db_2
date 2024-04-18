// place files you want to import through the `$lib` alias in this folder.
export class User {
    Id: number = 0;
    Nickname: string = "";
    Tier: number = 0;
    Email: string = "";
    Password: string = "";
}

export class Tier {
    Id: number = 0;
    Title: string = "";
    Price: number = 0;
    Limited: boolean = false;
}

export class Song {
    Id: number = 0;
    UserId: number = 0;
    Title: string = '';
    TimesPlayed: number = 0;
}

export class Playlist {
    Id: number = 0;
    UserId: number = 0;
    Title: string = '';
    IsPrivate: boolean = false;
}

export class PlaylistSong {
    PlaylistId: number = 0;
    SongId: number = 0;
}