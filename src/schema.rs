// Import the macro through its public re-export

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
    }
}