--! registered_objects : RegisteredObject(remaining_fuel?)
SELECT
    ro.id,
    ro.name, 
    ro.validation_status,
    ro.manoeuvrable, 
    ro.manoeuvre_latency,
    ro.avoidance_strategy,
    ro.manoeuvring_strategy,
    ro.remaining_fuel,
    ro.data_sharing_policy::text,
    (SELECT designator 
        FROM object_designators od 
        WHERE od.id = ro.id 
        ORDER BY catalogue ASC
        LIMIT 1) as designator
FROM
    registered_objects ro
WHERE
    ro.organisation_id = :organisation_id 
AND
    ro.organisation_id IN (SELECT get_orgs_for_app_user())
ORDER BY created_at DESC;

--! add_designator
INSERT INTO object_designators
    (registered_object_id, name, designator, catalogue)
VALUES
    (:registered_object_id, :name, :designator, :catalogue);

--! add_object(remaining_fuel?)
INSERT INTO registered_objects 
    (organisation_id, 
    name, 
    manoeuvrable, 
    manoeuvre_latency,
    avoidance_strategy,
    manoeuvring_strategy,
    remaining_fuel)
VALUES
    (:organisation_id, 
    :name, 
    :manoeuvrable, 
    :manoeuvre_latency,
    :avoidance_strategy,
    :manoeuvring_strategy,
    :remaining_fuel)
RETURNING id;

--! my_object
SELECT 
    id 
FROM 
    registered_objects 
WHERE 
    id = :object1_id or id = :object2_id
AND 
    organisation_id IN (SELECT get_orgs_for_app_user())
LIMIT 1;

-- Find an object either by it's cospar or norad.
--! find_object
SELECT 
    registered_object_id 
FROM 
    object_designators od
WHERE 
    (designator = :object_id AND catalogue = 'Cospar')
    or 
    (designator = :object_id AND catalogue = 'Satcat')
LIMIT 1;
