// place files you want to import through the `$lib` alias in this folder.
export class Tier {
    Id: number = 0;
    Title: string = "";
    Price: number = 0;
    Limited: boolean = false;

    public static fromSchema(t: TierSchema): Tier {
        return { Id: t.id, Title: t.title, Limited: t.is_limited, Price: t.price };
    }
}

export class User {
    Id: number = 0;
    Nickname: string = "";
    Tier: number = 0;
    Email: string = "";
    Password: string = "";

    public static fromSchema(u: UserSchema): User {
        return { Id: u.id, Nickname: u.nickname, Email: u.email, Password: u.password, Tier: u.tier_id };
    }
}

export class Song {
    Id: number = 0;
    UserId: number = 0;
    Title: string = '';
    TimesPlayed: number = 0;

    public static fromSchema(s: SongSchema): Song {
        return { Id: s.id, Title: s.title, TimesPlayed: s.times_played, UserId: s.user_id };
    }
}

export class Playlist {
    Id: number = 0;
    UserId: number = 0;
    Title: string = '';
    IsPrivate: boolean = false;

    public static fromSchema(p: PlaylistSchema): Playlist {
        return { Id: p.id, Title: p.title, IsPrivate: p.is_private, UserId: p.user_id };
    }
}

export class Link {
    PlaylistId: number = 0;
    SongId: number = 0;

    public static fromSchema(l: LinkSchema): Link {
        return { PlaylistId: l.playlist_id, SongId: l.song_id };
    }
}

export class TierSchema {
    id: number = 0;
    title: string = "";
    price: number = 0;
    is_limited: boolean = false;
}

export class UserSchema {
    id: number = 0;
    nickname: string = '';
    email: string = '';
    password: string = '';
    tier_id: number = 0;

    public static fromSchema(u: UserSchema): User {
        return { Id: u.id, Nickname: u.nickname, Email: u.email, Password: u.password, Tier: u.tier_id };
    }
}

export class SongSchema {
    id: number = 0;
    user_id: number = 0;
    title: string = '';
    times_played: number = 0;
}

export class PlaylistSchema {
    id: number = 0;
    user_id: number = 0;
    title: string = '';
    is_private: boolean = false;
}

export class LinkSchema {
    playlist_id: number = 0;
    song_id: number = 0;
}