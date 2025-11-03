-- This script runs automatically when the container is first created

-- Create additional database for testing
CREATE DATABASE collaborative_platform_test;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE collaborative_platform_test TO collab_user;

-- Enable extensions
\c collaborative_platform;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";  -- For fuzzy text search

\c collaborative_platform_test;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";  -- For fuzzy text search