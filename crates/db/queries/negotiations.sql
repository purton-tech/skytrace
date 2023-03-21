--: Negotiation()

--! negotiations : Negotiation
SELECT
    n.id,
    n.object1_id,
    (SELECT name FROM space_objects WHERE id = n.object1_id) as object1_name,
    object2_id,
    (SELECT name FROM space_objects WHERE id = n.object2_id) as object2_name,
    (SELECT resulting_status FROM time_line WHERE negotiation_id = n.id ORDER BY created_at DESC LIMIT 1) as status,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(n.created_at)::text) as created_at
FROM
    negotiations n
ORDER BY created_at DESC;

--! negotiation : Negotiation
SELECT
    n.id,
    n.object1_id,
    (SELECT name FROM space_objects WHERE id = n.object1_id) as object1_name,
    object2_id,
    (SELECT name FROM space_objects WHERE id = n.object2_id) as object2_name,
    (SELECT resulting_status FROM time_line WHERE negotiation_id = n.id ORDER BY created_at DESC LIMIT 1) as status,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(n.created_at)::text) as created_at
FROM
    negotiations n
WHERE 
    n.id = :negotiation_id;

--: TimeLine(initiator_email?)

--! time_line : TimeLine
SELECT  
    id,
    negotiation_id,
    (SELECT email from users where id = initiator_id) as initiator_email,
    action,
    subject,
    detail,
    resulting_status,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM
    time_line
WHERE   
    negotiation_id = :negotiation_id;

--! new_time_line
INSERT INTO time_line 
    (negotiation_id, initiator_id, subject, detail, action, resulting_status)
VALUES
    (:negotiation_id, :initiator_id, :subject, :detail, :action, :resulting_status);

--! new_negotiation
INSERT INTO negotiations
    (object1_id, object2_id, cdm_id)
VALUES
    (:object1_id, :object2_id, :cdm_id)
RETURNING id;