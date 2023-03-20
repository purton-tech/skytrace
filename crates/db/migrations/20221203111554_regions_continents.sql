-- migrate:up
CREATE TABLE continents
(
  id   SERIAL PRIMARY KEY,
  name VARCHAR(255) UNIQUE,
  code VARCHAR(8) UNIQUE
);
COMMENT ON TABLE continents IS 'Contains a list of continents.';

CREATE TABLE regions
(
  id           SERIAL PRIMARY KEY,
  name         VARCHAR(255) UNIQUE,
  code         VARCHAR(8) UNIQUE,
  continent_id INT NOT NULL,
  CONSTRAINT fk_continent FOREIGN KEY (continent_id) REFERENCES continents (id)
);
COMMENT ON TABLE regions IS 'Contains a list of geographic or artificial regions. (e.g. country or organisation)';

INSERT INTO continents (id, name, code)
VALUES
    (1, 'Asia', 'AS'),
    (2, 'Africa', 'AF'),
    (3, 'Europe', 'EU'),
    (4, 'North America', 'NA'),
    (5, 'South America', 'SA'),
    (6, 'Oceania', 'OC'),
    (7, 'Antarctica', 'AN'),
    (8, 'Multi Continental', 'MULTI')
    /* CAUTION
        This list should not be altered during runtime.
        Adding entries here, will overwrite any custom entries made.
        Always double check against production DB.
    */
ON CONFLICT (id) DO UPDATE SET code = EXCLUDED.code, name = EXCLUDED.name;

INSERT INTO regions (id, continent_id, code, name)
VALUES
    (1 , 1, 'AB', 'Arab Satellite Communications Organization'),
    (2 , 1, 'AC', 'Asiasat Corp'),
    (3 , 2, 'AGO', 'Angola'),
    (4 , 2, 'ALG', 'Algeria'),
    (5 , 5, 'ARGN', 'Argentina'),
    (6 , 3, 'ASRA', 'Austria'),
    (7 , 6, 'AUS', 'Australia'),
    (8 , 1, 'AZER', 'Azerbaijan'),
    (9 , 3, 'BEL', 'Belgium'),
    (10, 3, 'BELA', 'Belarus'),
    (11, 4, 'BERM', 'Bermuda'),
    (12, 1, 'BGD', 'Peoples Republic of Bangladesh'),
    (13, 3, 'BGR', 'Bulgaria'),
    (14, 5, 'BOL', 'Bolivia, Plurinational State of'),
    (15, 5, 'BRAZ', 'Brazil'),
    (16, 4, 'CA', 'Canada'),
    (17, 5, 'CHLE', 'Chile'),
    (18, 1, 'CIS', 'Commonwealth of Independent States'),
    (19, 5, 'COL', 'Colombia'),
    (20, 5, 'CRI', 'Republic of Costa Rica'),
    (21, 3, 'CZCH', 'Czechoslovakia'),
    (22, 3, 'DEN', 'Denmark'),
    (23, 5, 'ECU', 'Ecuador'),
    (24, 1, 'EGYP', 'Egypt'),
    (25, 3, 'ESA', 'European Space Agency'),
    (26, 3, 'ESRO', 'European Space Research Organization'),
    (27, 3, 'EST', 'Estonia'),
    (28, 3, 'EUME', 'European Organization for the Exploitation of Meteorological Satellites'),
    (29, 3, 'FIN', 'Finland'),
    (30, 3, 'FR', 'France'),
    (31, 3, 'GER', 'Germany'),
    (32, 2, 'GHA', 'Republic of Ghana'),
    (33, 3, 'GREC', 'Greece'),
    (34, 3, 'HUN', 'Hungary'),
    (35, 1, 'IND', 'India'),
    (36, 1, 'INDO', 'Indonesia'),
    (37, 1, 'IRAN', 'Iran, Islamic Republic of'),
    (38, 1, 'IRAQ', 'IRAQ'),
    (39, 1, 'ISRA', 'Israel'),
    (40, 3, 'IT', 'Italy'),
    (41, 1, 'JOR', 'Jordan'),
    (42, 1, 'JPN', 'Japan'),
    (43, 1, 'KAZ', 'Kazakhstan'),
    (44, 2, 'KEN', 'Republic of Kenya'),
    (45, 1, 'LAOS', 'Laos'),
    (46, 1, 'LKA', 'Democratic Socialist Republic of Sri Lanka'),
    (47, 3, 'LTU', 'Lithuania'),
    (48, 3, 'LUXE', 'Luxembourg'),
    (49, 2, 'MA', 'Morocco'),
    (50, 1, 'MALA', 'Malaysia'),
    (51, 5, 'MEX', 'Mexico'),
    (52, 1, 'MNG', 'Mongolia'),
    (53, 8, 'NATO', 'North Atlantic Treaty Organization'),
    (54, 3, 'NETH', 'Netherlands'),
    (55, 2, 'NIG', 'Nigeria'),
    (56, 1, 'NKOR', 'North Korea, Democratic Peoples Republic of'),
    (57, 3, 'NOR', 'Norway'),
    (58, 1, 'NPL', 'Nepal'),
    (59, 6, 'NZ', 'New Zealand'),
    (60, 1, 'PAKI', 'Pakistan'),
    (61, 5, 'PER', 'Peru'),
    (62, 3, 'POL', 'Poland'),
    (63, 3, 'POR', 'Portugal'),
    (64, 1, 'PRC', 'China'),
    (65, 2, 'RASC', 'Regional African Satellite Communications ORG'),
    (66, 1, 'ROC', 'Taiwan, Province of China'),
    (67, 3, 'ROM', 'Romania'),
    (68, 1, 'RP', 'Philippines'),
    (69, 2, 'RWA', 'Republic of Rwanda'),
    (70, 2, 'SAFR', 'South Africa'),
    (71, 1, 'SAUD', 'Saudi Arabia'),
    (72, 2, 'SDN', 'Republic of Sudan'),
    (73, 1, 'SING', 'Singapore'),
    (74, 1, 'SKOR', 'Korea, Republic of'),
    (75, 3, 'SPN', 'Spain'),
    (76, 3, 'SWED', 'Sweden'),
    (77, 3, 'SWTZ', 'Switzerland'),
    (78, 1, 'THAI', 'Thailand'),
    (79, 3, 'TURK', 'Turkey'),
    (80, 1, 'UAE', 'United Arab Emirates'),
    (81, 3, 'UK', 'United Kingdom'),
    (82, 1, 'UKN', 'Unknown'),
    (83, 3, 'UKR', 'Ukraine'),
    (84, 5, 'URY', 'Uruguay'),
    (85, 4, 'US', 'United States'),
    (86, 5, 'VENZ', 'Venezuela, Bolivarian Republic of'),
    (87, 1, 'VTNM', 'Vietnam')
    /* CAUTION
        This list should not be altered during runtime.
        Adding entries here, will overwrite any custom entries made.
        Always double check against production DB.
    */
ON CONFLICT (id) DO UPDATE SET continent_id = EXCLUDED.continent_id, code = EXCLUDED.code, name = EXCLUDED.name;

-- Give access to the application user, the application user has no access to 
-- The sessions table and therefore cannot fake a login.
GRANT SELECT, INSERT, UPDATE ON regions, continents TO trace_application;
GRANT USAGE, SELECT ON regions_id_seq, regions_id_seq TO trace_application;

-- Give access to the readonly user
GRANT SELECT ON regions, continents TO trace_readonly;
GRANT SELECT ON regions_id_seq, regions_id_seq TO trace_readonly;

-- migrate:down

DROP TABLE regions;
DROP TABLE continents;
