// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
    }
}

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
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 200]
        email -> Varchar,
        #[max_length = 1000]
        message -> Varchar,
    }
}

diesel::table! {
    logs (id) {
        id -> Text,
        #[max_length = 100]
        user_id -> Varchar,
        #[max_length = 100]
        text -> Varchar,
        #[max_length = 100]
        order_id -> Varchar,
        #[max_length = 100]
        place_id -> Varchar,
        created -> Timestamp,
    }
}

diesel::table! {
    module_types (id) {
        id -> Text,
        #[max_length = 100]
        title -> Varchar,
        types -> Int2,
        #[max_length = 500]
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    modules (id) {
        id -> Text,
        #[max_length = 100]
        title -> Varchar,
        types -> Int2,
        #[max_length = 100]
        place_id -> Varchar,
        #[max_length = 100]
        type_id -> Varchar,
        price -> Int4,
        _width -> Int2,
        _height -> Int2,
        _left -> Float8,
        _top -> Float8,
        _angle -> Float8,
        #[max_length = 10]
        font_color -> Varchar,
        #[max_length = 10]
        font_size -> Varchar,
        #[max_length = 10]
        back_color -> Varchar,
        #[max_length = 500]
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    orders (id) {
        id -> Text,
        #[max_length = 100]
        title -> Varchar,
        types -> Int2,
        #[max_length = 100]
        place_id -> Varchar,
        #[max_length = 100]
        object_id -> Varchar,
        created -> Timestamp,
        #[max_length = 100]
        user_id -> Varchar,
        price -> Int4,
        #[max_length = 100]
        time_start -> Varchar,
        #[max_length = 100]
        time_end -> Varchar,
    }
}

diesel::table! {
    partners (id) {
        id -> Text,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 100]
        inn -> Varchar,
        types -> Int2,
        created -> Timestamp,
        #[max_length = 100]
        user_id -> Varchar,
    }
}

diesel::table! {
    place_managers (id) {
        id -> Text,
        #[max_length = 100]
        user_id -> Varchar,
        #[max_length = 100]
        place_id -> Varchar,
    }
}

diesel::table! {
    place_types (id) {
        id -> Text,
        #[max_length = 100]
        title -> Varchar,
    }
}

diesel::table! {
    places (id) {
        id -> Text,
        #[max_length = 100]
        title -> Varchar,
        types -> Int2,
        created -> Timestamp,
        #[max_length = 100]
        user_id -> Varchar,
        city_id -> Int4,
        type_id -> Int2,
        #[max_length = 500]
        image -> Nullable<Varchar>,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    regions (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        country_id -> Int4,
        timezone_id -> Nullable<Int4>,
        #[max_length = 100]
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
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 1000]
        password -> Varchar,
        perm -> Int2,
        level -> Int2,
        #[max_length = 500]
        image -> Nullable<Varchar>,
        uuid -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cities,
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
    regions,
    times,
    users,
);
