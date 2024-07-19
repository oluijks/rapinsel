-- Add up migration script here

CREATE TABLE users (
    id SMALLSERIAL PRIMARY KEY,
    uuid UUID NOT NULL DEFAULT (uuid_generate_v4 ()),
    name VARCHAR(255),
    email VARCHAR(255) NOT NULL,
    encrypted_password TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);