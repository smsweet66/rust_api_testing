-- Your SQL goes here
create table if not exists users (
	id serial primary key,
	name varchar(255) not null,
	email varchar(255) not null unique,
	password varchar(255) not null,
	created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

create table if not exists profiles (
	id serial primary key,
	user_id integer not null references users(id) on delete cascade,
	name varchar(255) not null,
	body_sizes varchar(255) not null,
	created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);