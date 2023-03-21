--: OrbitData()

--! filter_orbit_data : OrbitData
SELECT
    id,
    organisation_id,
    coalesce(json->'body'->'segment'->0->'metadata'->>'object_name', '') as object_name,
    coalesce(json->'body'->'segment'->0->'metadata'->>'object_id', '') as object_id,
    coalesce(json->'body'->'segment'->0->'metadata' ->> 'center_name', '') as originator,
    encode(signature::bytea, 'hex') as signature,
    confidentiality,
    updated_at,
    created_at
FROM
    orbit_data
WHERE
    id < :seek_pagination
ORDER BY id DESC
LIMIT :page_size;

--! add_orbit_data
INSERT INTO orbit_data 
    (organisation_id, protobuf, json, signature, confidentiality)
VALUES
    (:organisation_id, :protobuf, :json, :signature, :confidentiality);