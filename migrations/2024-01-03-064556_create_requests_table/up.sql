CREATE TABLE requests (
    id uuid PRIMARY KEY,
    event_id uuid REFERENCES events(id),
    body TEXT,
    body_used BOOLEAN,
    cache VARCHAR(255),
    credentials VARCHAR(255),
    destination VARCHAR(255),
    headers JSONB,
    integrity VARCHAR(255),
    method VARCHAR(255),
    mode VARCHAR(255),
    redirect VARCHAR(255),
    referrer VARCHAR(255),
    referrer_policy VARCHAR(255),
    url VARCHAR(255),
    signal JSONB
);
