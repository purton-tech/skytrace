-- migrate:up

-- The very simplest RBAC implementation, the roles get added to the organisation_users table
-- as users are added to an org.

CREATE TYPE role AS ENUM (
    'Administrator', 
    'Collaborator', 
    'Negotiator', 
    'SystemAdministrator'
);
COMMENT ON TYPE role IS 'Users have roles, they can be managers or administrators etc.';

CREATE TYPE permission AS ENUM (
    -- The ManageTeam permission gives the user thee ability to invite team members, 
    -- delete team members and change the team name
    'ManageTeam',
    -- The negotiate permission allows the user to administer negotiations
    'Negotiate'
);
COMMENT ON TYPE permission IS 'A permission gives the user the ability to do something. i.e. Manage users.';

CREATE TABLE roles_permissions (
    role role NOT NULL,
    permission permission NOT NULL,

    PRIMARY KEY (role, permission)
);
COMMENT ON TABLE roles_permissions IS 'Maps roles to permissions. i.e. a role can have multiple permissions.';

INSERT INTO roles_permissions VALUES('Administrator', 'ManageTeam');
INSERT INTO roles_permissions VALUES('Negotiator', 'Negotiate');


-- Give access to the application user.
GRANT SELECT ON roles_permissions TO trace_application;

-- Give access to the readonly user
GRANT SELECT ON roles_permissions TO trace_readonly;

-- migrate:down
DROP TABLE roles_permissions;
DROP TYPE role;
DROP TYPE permission;