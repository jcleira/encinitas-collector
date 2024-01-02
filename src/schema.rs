// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        #[max_length = 255]
        browser_id -> Nullable<Varchar>,
        #[max_length = 255]
        client_id -> Nullable<Varchar>,
        handled -> Nullable<Bool>,
        #[max_length = 255]
        replaces_client_id -> Nullable<Varchar>,
        #[max_length = 255]
        resulting_client_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    requests (id) {
        id -> Int4,
        event_id -> Nullable<Int4>,
        body -> Nullable<Text>,
        body_used -> Nullable<Bool>,
        #[max_length = 255]
        cache -> Nullable<Varchar>,
        #[max_length = 255]
        credentials -> Nullable<Varchar>,
        #[max_length = 255]
        destination -> Nullable<Varchar>,
        headers -> Nullable<Jsonb>,
        #[max_length = 255]
        integrity -> Nullable<Varchar>,
        #[max_length = 255]
        method -> Nullable<Varchar>,
        #[max_length = 255]
        mode -> Nullable<Varchar>,
        #[max_length = 255]
        redirect -> Nullable<Varchar>,
        #[max_length = 255]
        referrer -> Nullable<Varchar>,
        #[max_length = 255]
        referrer_policy -> Nullable<Varchar>,
        #[max_length = 255]
        url -> Nullable<Varchar>,
        signal -> Nullable<Text>,
    }
}

diesel::table! {
    responses (id) {
        id -> Int4,
        event_id -> Nullable<Int4>,
        body -> Nullable<Text>,
        body_used -> Nullable<Bool>,
        headers -> Nullable<Jsonb>,
        ok -> Nullable<Bool>,
        redirected -> Nullable<Bool>,
        status -> Nullable<Int4>,
        #[max_length = 255]
        status_text -> Nullable<Varchar>,
        #[max_length = 255]
        response_type -> Nullable<Varchar>,
        #[max_length = 255]
        url -> Nullable<Varchar>,
    }
}

diesel::joinable!(requests -> events (event_id));
diesel::joinable!(responses -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    requests,
    responses,
);
