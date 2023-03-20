-- migrate:up

CREATE TYPE SHARING_RULE_TYPE AS ENUM ('allow', 'deny');

COMMENT ON TABLE affiliations IS 'Contains definitions of different user affiliations.';

CREATE TABLE data_sharing_rules
(
  id                 SERIAL PRIMARY KEY,
  organisation_id            INT               NOT NULL,
  object_id          INT               NULL,
  name               VARCHAR(255),
  description        VARCHAR(512),
  type               SHARING_RULE_TYPE NOT NULL,
  hie_only           BOOLEAN           NOT NULL DEFAULT false,
  applicable_from    TIMESTAMP                  DEFAULT NULL,
  applicable_until   TIMESTAMP                  DEFAULT NULL,
  restricted_user_id INT                        DEFAULT NULL,
  continent_id       INT                        DEFAULT NULL,
  region_id          INT                        DEFAULT NULL,
  affiliation_id     INT                        DEFAULT NULL,
  created_at         TIMESTAMP         NOT NULL DEFAULT NOW(),
  updated_at         TIMESTAMP         NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_organisation FOREIGN KEY (organisation_id) REFERENCES organisations (id),
  CONSTRAINT fk_object FOREIGN KEY (object_id) REFERENCES registered_objects (id),
  CONSTRAINT fk_restricted_user FOREIGN KEY (restricted_user_id) REFERENCES users (id),
  CONSTRAINT fk_continent FOREIGN KEY (continent_id) REFERENCES continents (id),
  CONSTRAINT fk_region FOREIGN KEY (region_id) REFERENCES regions (id),
  CONSTRAINT fk_affiliation FOREIGN KEY (affiliation_id) REFERENCES affiliations (id)
);

CREATE INDEX idx_data_sharing_rule_org ON data_sharing_rules (organisation_id);
CREATE INDEX idx_data_sharing_rule_object ON data_sharing_rules (object_id);
CREATE INDEX idx_data_sharing_rule_restricted_user ON data_sharing_rules (restricted_user_id);
CREATE INDEX idx_data_sharing_rule_continent ON data_sharing_rules (continent_id);
CREATE INDEX idx_data_sharing_rule_region ON data_sharing_rules (region_id);
CREATE INDEX idx_data_sharing_rule_affiliation ON data_sharing_rules (affiliation_id);

COMMENT ON TABLE data_sharing_rules IS 'Contains user defined data sharing policies.';
COMMENT ON COLUMN data_sharing_rules.organisation_id IS 'ID of the organisation this rule belongs to.';
COMMENT ON COLUMN data_sharing_rules.object_id IS 'Optional - ID of the object, this rule applies to.';
COMMENT ON COLUMN data_sharing_rules.type IS 'Type of this rule (allow or deny).';
COMMENT ON COLUMN data_sharing_rules.hie_only IS 'If true, this rule applies to high interest events only.';
COMMENT ON COLUMN data_sharing_rules.applicable_from IS 'Optional - A timestamp from which this rule is applicable.';
COMMENT ON COLUMN data_sharing_rules.applicable_until IS 'Optional - A timestamp until which this rule is applicable.';
COMMENT ON COLUMN data_sharing_rules.restricted_user_id IS 'Optional - ID of the user, this rule applies to.';
COMMENT ON COLUMN data_sharing_rules.continent_id IS 'Optional - ID of the continent, this rule applies to.';
COMMENT ON COLUMN data_sharing_rules.region_id IS 'Optional - ID of the region, this rule applies to.';
COMMENT ON COLUMN data_sharing_rules.affiliation_id IS 'Optional - ID of the user affiliation, this rule applies to.';

-- Give access to the application user, the application user has no access to 
-- The sessions table and therefore cannot fake a login.
GRANT SELECT, INSERT, UPDATE ON affiliations TO trace_application;
GRANT USAGE, SELECT ON affiliations_id_seq TO trace_application;

-- Give access to the readonly user
GRANT SELECT ON affiliations TO trace_readonly;
GRANT SELECT ON affiliations_id_seq TO trace_readonly;


-- migrate:down
DROP TABLE data_sharing_rules;
DROP TYPE SHARING_RULE_TYPE;
