--: TrackingData()

--! filter_tracking_data : TrackingData
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
    confidentiality,
    updated_at,
    created_at
FROM
    tracking_data
WHERE
    id < :seek_pagination
ORDER BY id DESC
LIMIT :page_size;

--! add_tracking_data
INSERT INTO tracking_data 
    (organisation_id, protobuf, json, signature, confidentiality)
VALUES
    (:organisation_id, :protobuf, :json, :signature, :confidentiality);