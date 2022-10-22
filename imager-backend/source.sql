drop table if exists image;
create table image(
    id bigint(20) unsigned not null auto_increment,
    url varchar(255) not null,
    primary key(id)
);
insert into image(url) values('https://pictu1.1plq.com/p1587/2021/02/21/p1587-5434-72063.jpg');
insert into image(url) values('https://pictu1.1plq.com/p1587/2021/02/21/p1587-5434-72064.jpg');
