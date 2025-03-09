// @generated automatically by Diesel CLI.

diesel::table! {
    email_verification_token (id) {
        id -> Bytea,
        email -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    feedbacks (id) {
        id -> Text,
        username -> Varchar,
        email -> Varchar,
        message -> Varchar,
    }
}

diesel::table! {
    logs (id) {
        id -> Text,
        user_id -> Varchar,
        text -> Varchar,
        order_id -> Varchar,
        place_id -> Varchar,
        created -> Timestamp,
    }
}

diesel::table! {
    module_types (id) {
        id -> Text,
        title -> Varchar,
        types -> Int2,
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    modules (id) {
        id -> Text,
        title -> Varchar,
        types -> Int2,
        place_id -> Varchar,
        type_id -> Varchar,
        price -> Int4,
        _width -> Int2,
        _height -> Int2,
        _left -> Float8,
        _top -> Float8,
        _angle -> Float8,
        font_color -> Varchar,
        font_size -> Varchar,
        back_color -> Varchar,
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    orders (id) {
        id -> Text,
        title -> Varchar,
        types -> Int2,
        place_id -> Varchar,
        object_id -> Varchar,
        created -> Timestamp,
        user_id -> Varchar,
        price -> Int4,
        time_start -> Varchar,
        time_end -> Varchar,
    }
}

diesel::table! {
    partners (id) {
        id -> Text,
        title -> Varchar,
        inn -> Varchar,
        types -> Int2,
        created -> Timestamp,
        user_id -> Varchar,
    }
}

diesel::table! {
    place_managers (id) {
        id -> Text,
        user_id -> Varchar,
        place_id -> Varchar,
    }
}

diesel::table! {
    place_types (id) {
        id -> Text,
        title -> Varchar,
    }
}

diesel::table! {
    places (id) {
        id -> Text,
        title -> Varchar,
        types -> Int2,
        created -> Timestamp,
        user_id -> Varchar,
        type_id -> Varchar,
        image -> Nullable<Varchar>,
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    times (id) {
        id -> Text,
        time -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        perm -> Int2,
        level -> Int2,
        image -> Nullable<Varchar>,
        uuid -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    email_verification_token,
    feedbacks,
    logs,
    module_types,
    modules,
    orders,
    partners,
    place_managers,
    place_types,
    places,
    times,
    users,
);
