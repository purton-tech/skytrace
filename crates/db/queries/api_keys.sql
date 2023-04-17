--: ApiKey()

--! api_keys : ApiKey
SELECT
    id,
    name,
    user_id,
    api_key,
    created_at
FROM
    api_keys
WHERE 
    :organisation_id IN 
        (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user())
ORDER BY created_at DESC;

--! new_api_key
INSERT INTO api_keys 
    (user_id, name, api_key)
VALUES
    (:user_id, :name, :api_key);

--! find_api_key : ApiKey
SELECT
    id,
    name,
    user_id,
    api_key,
    created_at
FROM
    api_keys
WHERE
    api_key = :api_key;

