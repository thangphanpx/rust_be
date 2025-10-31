-- Remove sample data
DELETE FROM posts WHERE title IN ('Welcome to our API', 'Getting Started Guide');
DELETE FROM users WHERE email = 'admin@example.com';