drop table if exists image;
create table image(
    id bigint(20) unsigned not null auto_increment,
    url varchar(255) not null,
    primary key(id)
)
