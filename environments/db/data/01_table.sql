\c recruitment;

CREATE SCHEMA 採用;

CREATE TABLE 採用.候補者(
    Id UUID PRIMARY KEY,
    名前 VARCHAR(100) NOT NULL,
    年齢 SMALLINT
);

CREATE TABLE 採用.候補者ステータス(
    候補者Id UUID PRIMARY KEY,
    選考ステータス VARCHAR(50) NOT NULL,
    選考プロセスステータス VARCHAR(50) NOT NULL,

    FOREIGN KEY (候補者Id) REFERENCES 採用.候補者(Id)
);