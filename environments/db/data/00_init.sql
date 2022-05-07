CREATE USER developer WITH PASSWORD 'developer' SUPERUSER;
CREATE DATABASE recruitment owner developer encoding 'UTF8';
GRANT ALL PRIVILEGES ON DATABASE recruitment TO developer;

\c recruitment;

SET statement_timeout = 0;
SET lock_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;

ALTER SYSTEM SET max_connections = 5000;
ALTER SYSTEM RESET shared_buffers;