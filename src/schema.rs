table! {
    activities (id) {
        id -> Integer,
        user_id -> Text,
        name -> Text,
        short_description -> Nullable<Text>,
        start_date -> Integer,
        end_date -> Integer,
        category_id -> Integer,
        status_id -> Integer,
    }
}

table! {
    categories (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    statuses (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Nullable<Text>,
        dob -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    activities,
    categories,
    statuses,
    users,
);
