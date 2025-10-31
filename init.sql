-- Initialize database with sample data

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create posts table
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert sample user
INSERT INTO users (email, username, password_hash, full_name) 
VALUES (
    'admin@example.com', 
    'admin', 
    '$argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$123456', -- This is a dummy hash, replace with real one
    'Administrator'
) ON CONFLICT (email) DO NOTHING;

-- Insert sample posts
INSERT INTO posts (title, content, user_id, is_published) 
SELECT 
    'Welcome to our API',
    'This is a sample post created during database initialization.',
    u.id,
    true
FROM users u 
WHERE u.email = 'admin@example.com'
ON CONFLICT DO NOTHING;