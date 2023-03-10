-- Users
create table users (
    id text default gen_random_ulid () not null primary key,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null,
    email text not null
);

create unique index users__email__unique on users (email);

create trigger sync_users_updated_at
    before update on users for each row
    execute procedure sync_updated_at ();