--! user : (first_name?, last_name?)
SELECT id, email, first_name, last_name FROM users WHERE id = :user_id;

--! get_by_email : (first_name?, last_name?)
SELECT 
    id, email, ecdsa_public_key, first_name, last_name
FROM 
    users
WHERE
    email = :email;

--! set_name(first_name, last_name, current_user_id)
UPDATE
    users
SET 
    first_name = :first_name, last_name = :last_name
WHERE
    id = :current_user_id;