-- Your SQL goes here
-- Create tables if they do not exist
CREATE TABLE IF NOT EXISTS books (
    id SERIAL PRIMARY KEY,  -- 书本 ID
    title VARCHAR(255) NOT NULL, -- 书本标题
    author VARCHAR(255), -- 作者
    description TEXT,   -- 书本描述
    status INT, -- 0: Inactive, 1: Active
    rating FLOAT,   -- 评分
    added_date DATE NOT NULL,  -- 添加日期
    cover_url VARCHAR(255) NOT NULL,  -- 书本封面的 URL
    extension VARCHAR(255) NOT NULL  -- 书本文件的扩展名
);

-- Insert test data
INSERT INTO books (title, author, description, status, rating, added_date, cover_url, extension) VALUES
('Book One', 'Author One', 'Description One', 1, 4.5, '2023-01-01', 'cover_url_1', 'pdf'),
('Book Two', 'Author Two', 'Description Two', 1, 4.0, '2023-02-01', 'cover_url_2', 'epub');

-- Your SQL goes here
CREATE TABLE users (
    uid UUID PRIMARY KEY,
    login VARCHAR NOT NULL,
    name VARCHAR,
    avatar VARCHAR,
    email VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    github_id BIGINT,
    casdoor_id VARCHAR
);

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,  -- 分类 ID
    name VARCHAR(255) NOT NULL  -- 分类名称
);

INSERT INTO categories (name) VALUES
('Fiction'),
('Non-Fiction');
