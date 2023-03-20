-- migrate:up

CREATE TABLE affiliations
(
  id          SERIAL PRIMARY KEY,
  name        VARCHAR(255) UNIQUE,
  description VARCHAR(512)
);

COMMENT ON TABLE affiliations IS 'Contains definitions of different user affiliations.';


INSERT INTO affiliations (id, name, description)
VALUES
    (1, 'Commercial', 'Commercial organisation'),
    (2, 'Governmental', 'Governmental organisation'),
    (3, 'Military', 'Military organisation'),
    (4, 'Academia', 'Academical organisation (e.g. university)')
    /* CAUTION
        This list should not be altered during runtime.
        Adding entries here, will overwrite any custom entries made.
        Always double check against production DB.
    */
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, description = EXCLUDED.description;

-- Give access to the application user, the application user has no access to 
-- The sessions table and therefore cannot fake a login.
GRANT SELECT, INSERT, UPDATE ON affiliations TO trace_application;
GRANT USAGE, SELECT ON affiliations_id_seq TO trace_application;

-- Give access to the readonly user
GRANT SELECT ON affiliations TO trace_readonly;
GRANT SELECT ON affiliations_id_seq TO trace_readonly;

-- migrate:down

DROP TABLE affiliations;