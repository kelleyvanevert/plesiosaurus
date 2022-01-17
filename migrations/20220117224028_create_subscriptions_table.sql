
create extension if not exists "uuid-ossp";

create table subscriptions(
  id uuid not null default uuid_generate_v4 () primary key,
  email text not null unique,
  name text not null,
  subscribed_at timestamptz not null
);
