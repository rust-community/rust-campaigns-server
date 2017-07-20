CREATE TABLE campaigns (
   id bigserial not null primary key,
   title varchar(256) not null,
   description varchar(1024),
   start_date timestamp not null,
   end_date timestamp,
   click_url varchar(2048) not null
)
