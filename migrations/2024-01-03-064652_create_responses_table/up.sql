CREATE TABLE responses (
    id uuid PRIMARY KEY,
    event_id uuid REFERENCES events(id),
    body TEXT,
    body_used BOOLEAN,
    headers JSONB,
    ok BOOLEAN,
    redirected BOOLEAN,
    status INTEGER,
    status_text VARCHAR(255),
    response_type VARCHAR(255),
    url VARCHAR(255)
);
