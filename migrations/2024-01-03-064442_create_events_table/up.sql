CREATE TABLE events (
    id uuid PRIMARY KEY,
    browser_id VARCHAR(255),
    client_id VARCHAR(255),
    handled JSONB,
    replaces_client_id VARCHAR(255),
    resulting_client_id VARCHAR(255)
);
