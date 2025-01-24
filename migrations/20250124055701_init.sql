-- Add migration script here
-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    user_id INTEGER NOT NULL references users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE tick_info (
    id SERIAL PRIMARY KEY,
    tick BIGINT NOT NULL,
    duration BIGINT NOT NULL,
    epoch INTEGER NOT NULL,
    initial_tick BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
