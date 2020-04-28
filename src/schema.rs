table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    listings (id) {
        id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        passphrase -> Varchar,
        enabled -> Bool,
    }
}

joinable!(listings -> games (game_id));
joinable!(listings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    games,
    listings,
    users,
);
