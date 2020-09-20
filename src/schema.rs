table! {
    activities (id) {
        id -> Integer,
        user_id -> Text,
        name -> Text,
        short_description -> Nullable<Text>,
        start_date -> Integer,
        end_date -> Integer,
        category_id -> Text,
        status_id -> Text,
    }
}

table! {
    categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    statuses (id) {
        id -> Text,
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
