--: Conjunction()

--! filter_conjunctions(pc_greater?, pc_less?, object1_name_like?, object2_name_like?) : Conjunction
SELECT
    id,
    organisation_id,
    coalesce(json->'body'->'segment'->0->'metadata'->>'object_name', '') as first_object,
    coalesce(json->'body'->'segment'->1->'metadata'->>'object_name', '') as secondary_object,
    coalesce(json->'body'->'segment'->0->'metadata' ->> 'object_designator', '') as first_object_id,
    coalesce(json->'body'->'segment'->1->'metadata' ->> 'object_designator', '') as secondary_object_id,
    coalesce((json -> 'body' -> 'relative_metadata_data' ->> 'collision_probability')::float, 0.0) as collision_probability,
    coalesce((json -> 'body' -> 'relative_metadata_data' -> 'miss_distance' ->> 'value')::float, 0.0) as miss_distance,
    coalesce(json -> 'body' -> 'relative_metadata_data' ->> 'tca', '') as time_of_closest_approach,
    encode(signature::bytea, 'hex') as signature,
    sharing_policy,
    updated_at,
    created_at
FROM
    conjunctions
WHERE
    (
        :pc_greater::float IS NULL 
        OR 
        coalesce((json -> 'relative_metadata' ->> 'collision_probability')::float, 0.0) > :pc_greater
    )
    AND
    (
        :pc_less::float IS NULL 
        OR 
        coalesce((json -> 'relative_metadata' ->> 'collision_probability')::float, 0.0) < :pc_less
    )
    AND
    (
        :object1_name_like::varchar IS NULL 
        OR 
        LOWER(coalesce(json -> 'object1_metadata' ->> 'object_name', '')) LIKE '%' || LOWER(:object1_name_like) || '%'
    )
    AND
    (
        :object2_name_like::varchar IS NULL 
        OR 
        LOWER(coalesce(json -> 'object2_metadata' ->> 'object_name', '')) LIKE '%' || LOWER(:object2_name_like) || '%'
    )
    AND
    id < :seek_pagination
ORDER BY id DESC
LIMIT :page_size;

--! add_conjunction
INSERT INTO conjunctions 
    (organisation_id, protobuf, json, signature, sharing_policy)
VALUES
    (:organisation_id, :protobuf, :json, :signature, :sharing_policy);

--! count_conjunction_by_message_id 
SELECT
    COUNT(*) AS cdm_count
FROM
    conjunctions
WHERE 
    json -> 'header' ->> 'message_id' = :message_id;

--! conjunctions_with_no_negotiation : Conjunction
SELECT DISTINCT
    id,
    organisation_id,
    coalesce(json->'body'->'segment'->0->'metadata'->>'object_name', '') as first_object,
    coalesce(json->'body'->'segment'->1->'metadata'->>'object_name', '') as secondary_object,
    coalesce(json->'body'->'segment'->0->'metadata' ->> 'object_designator', '') as first_object_id,
    coalesce(json->'body'->'segment'->1->'metadata' ->> 'object_designator', '') as secondary_object_id,
    coalesce((json -> 'body' -> 'relative_metadata_data' ->> 'collision_probability')::float, 0.0) as collision_probability,
    coalesce((json -> 'body' -> 'relative_metadata_data' -> 'miss_distance' ->> 'value')::float, 0.0) as miss_distance,
    coalesce(json -> 'body' -> 'relative_metadata_data' ->> 'tca', '') as time_of_closest_approach,
    encode(signature::bytea, 'hex') as signature,
    sharing_policy,
    updated_at,
    created_at
FROM
    conjunctions
WHERE
    id NOT IN (SELECT cdm_id FROM negotiations)
ORDER BY created_at DESC;