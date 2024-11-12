CREATE DATABASE mm_git_db;
\c mm_git_db;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE acl_role AS ENUM ('read', 'write');

CREATE TABLE acl (
    repo_id uuid NOT NULL,
    user_id uuid NOT NULL,
    role acl_role NOT NULL
);

CREATE TABLE auth (
    key_fingerprint bytea NOT NULL,
    user_id uuid NOT NULL,
)

