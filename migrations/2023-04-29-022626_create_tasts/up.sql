-- Your SQL goes here
create table if not exists users (
	id integer primary key autoincrement not null,
	name varchar(255) not null unique,
	email varchar(255) not null unique,
	password varchar(255) not null,
	created_at timestamp default current_timestamp not null,
	updated_at timestamp default current_timestamp not null
);