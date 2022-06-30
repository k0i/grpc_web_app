-- Add migration script here
CREATE TABLE news (
  news_id     varchar(26)   NOT NULL ,
  PRIMARY KEY (news_id),
  title       varchar(255)  NOT NULL,
  content     text          NOT NULL,
  sourced_by  varchar(255)  DEFAULT NULL,
  created_at  timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at  timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at  timestamp     DEFAULT NULL
) ;
