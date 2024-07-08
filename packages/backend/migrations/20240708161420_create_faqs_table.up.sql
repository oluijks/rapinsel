-- Add up migration script here

CREATE TABLE faqs (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4 ()),
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);