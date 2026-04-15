-- Add up migration script here
CREATE TABLE public.refresh_tokens (
    id BIGSERIAL NOT NULL,
    user_id BIGINT NOT NULL,
    token VARCHAR NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT refresh_tokens_pk PRIMARY KEY (id),
    CONSTRAINT refresh_tokens_unique UNIQUE (token),
    CONSTRAINT refresh_tokens_user_fk FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);
