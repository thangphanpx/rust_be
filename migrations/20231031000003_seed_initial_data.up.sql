-- Insert sample admin user
INSERT INTO users (email, username, password_hash, full_name) 
VALUES (
    'admin@example.com', 
    'admin', 
    'hashed_password123', -- This would be properly hashed in real application
    'Administrator'
) ON CONFLICT (email) DO NOTHING;

-- Insert sample posts
INSERT INTO posts (title, content, user_id, is_published) 
SELECT 
    'Welcome to our API',
    'This is a sample post created during database initialization. This post demonstrates the basic functionality of our blog API.',
    u.id,
    true
FROM users u 
WHERE u.email = 'admin@example.com'
AND NOT EXISTS (SELECT 1 FROM posts WHERE title = 'Welcome to our API');

INSERT INTO posts (title, content, user_id, is_published) 
SELECT 
    'Getting Started Guide',
    'Here you will find everything you need to know to get started with our API. Check out the documentation and examples.',
    u.id,
    true
FROM users u 
WHERE u.email = 'admin@example.com'
AND NOT EXISTS (SELECT 1 FROM posts WHERE title = 'Getting Started Guide');