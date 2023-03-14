create table sch.main (
    id    varchar(255) not null primary key,
    value text         not null
);

create table sch.log (
    id        SERIAL NOT NULL primary key,
    date_time TIMESTAMP default now(),
    value     text   not null
);
