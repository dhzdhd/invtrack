use diesel::table;

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        published -> Bool,
    }
}
