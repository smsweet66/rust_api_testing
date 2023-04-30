-- Your SQL goes here
create table if not exists users (
	uid integer primary key autoincrement not null,
	name varchar(255) not null,
	email varchar(255) not null unique,
	password varchar(255) not null,
	created_at timestamp default current_timestamp not null,
	updated_at timestamp default current_timestamp not null
);

create table if not exists profiles (
	pid integer primary key autoincrement not null,
	uid integer not null,
	name varchar(255) not null,
	body_sizes varchar(255) not null,
	created_at timestamp default current_timestamp not null,
	updated_at timestamp default current_timestamp not null,
	foreign key (uid) references users(uid)
);