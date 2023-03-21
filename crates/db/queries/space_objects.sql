--! space_objects : RegisteredObject(remaining_fuel_kg?)
SELECT
    so.id,
    so.name, 
    so.validation_status,
    so.manoeuvrable, 
    so.manoeuvre_latency,
    so.avoidance_strategy::text,
    so.manoeuvre_strategy::text,
    so.remaining_fuel_kg,
    so.confidentiality::text,
    (SELECT object_identifier 
        FROM object_identifiers oi 
        WHERE oi.id = so.id 
        ORDER BY identifier_type ASC
        LIMIT 1) as object_identifier
FROM
    space_objects so
WHERE
    so.organisation_id = :organisation_id 
AND
    so.organisation_id IN (SELECT get_orgs_for_app_user())
ORDER BY created_at DESC;

--! add_designator
INSERT INTO object_identifiers
    (space_object_id, object_identifier, identifier_type)
VALUES
    (:space_object_id, :object_identifier, :identifier_type);

--! add_object(remaining_fuel_kg?)
INSERT INTO space_objects 
    (organisation_id, 
    name, 
    manoeuvrable, 
    manoeuvre_latency,
    avoidance_strategy,
    manoeuvre_strategy,
    remaining_fuel_kg)
VALUES
    (:organisation_id, 
    :name, 
    :manoeuvrable, 
    :manoeuvre_latency,
    :avoidance_strategy,
    :manoeuvre_strategy,
    :remaining_fuel_kg)
RETURNING id;

--! my_object
SELECT 
    id 
FROM 
    space_objects 
WHERE 
    id = :object1_id or id = :object2_id
AND 
    organisation_id IN (SELECT get_orgs_for_app_user())
LIMIT 1;

-- Find an object either by it's cospar or norad.
--! find_object
SELECT 
    space_object_id 
FROM 
    object_identifiers oi
WHERE 
    (object_identifier = :object_id AND identifier_type = 'Cospar')
    or 
    (object_identifier = :object_id AND identifier_type = 'Satcat')
LIMIT 1;
