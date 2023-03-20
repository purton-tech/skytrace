-- migrate:up

/*
   State of ongoing manoeuvre and manoeuvre negotiation.

   The regular lifecycle for a negotiation is that it will transition
   through the following states:

   * `MAN_NEGOTIATE` for the CAM negotiation, followed by `MAN_REQUIRED` or
     `MAN_NOT_REQUIRED` depending on the negotiation outcome.
   * From `MAN_NOT_REQUIRED` to `MAN_NOT_EXECUTED` once the TCA is reached.
   * From `MAN_REQUIRED` in sequence to `MAN_PLANNED`, `MAN_SCREENED`,
     `MAN_SIGNED` and `MAN_UPLINKED`. `MAN_PLANNED` and `MAN_SCREENED` can be
     skipped if post-trajectory screening is performed by the O/O itself.
   * From `MAN_UPLINKED` to `MAN_EXECUTED` once the TCA is reached.
   * From `MAN_UPLINKED` or `MAN_EXECUTED` to `MAN_FAILED` if the assigned O/O
     indicates that the planned manoeuvre has failed.

   In addition, a transition from `MAN_SCREENED` to `MAN_REQUIRED` is possible
   to enable re-planning of a manoeuvre if post-trajectory screening revealed
   issues, and a accepted delta negotiation request will transition back to
   `MAN_NEGOTIATE` state.
*/
CREATE TYPE negotiation_status AS ENUM (
   /* System expects negotiation inputs for high-interest conjunction events. */
   'ManoeuvreNegotiate', 
   /* 
      Outcome of negotiation has determined that a collision avoidance manoeuvre 
      (CAM) is required. Manoeuvring responsibility has been assigned.
   */
   'ManoeuvreRequired', 
   /* The CAM assignee has confirmed the plan to manoeuvre. */
   'ManoeuvrePlanned', 
   /* The manoeuvre has been screened by a CAP. */
   'ManoeuvreScreened', 
   /* The CAM assignee has approved the manoeuvre. */
   'ManoeuvreSigned', 
   /* The CAM assignee has allocated the manoeuvre for uplink to the space object. */
   'ManoeuvreUplinked',
   /* TCA has been reached and manoeuvre has been executed (final state). */
   'ManoeuvreExecuted',
   /* TCA has been reached and manoeuvre has failed (final state). */
   'ManoeuvreFailed',
   /* Outcome of negotiation has determined that no avoidance manoeuvre is required. */
   'ManoeuvreNotRequired',
   /* TCA has been reached and no manoeuvre has been executed (final state). */
   'ManoeuvreNotExecuted'
);

CREATE TYPE negotiation_action AS ENUM (
   /*
      When a High Interest Event (HIE) is detected then negotiation entry is created in the
      table and a time line entry is crated with the intial action as NegotiationTriggered
   */
   'NegotiationTriggered',
   /*
      Request a delta negotiation if collision risk changes after manoeuvre
      responsibility has been assigned. Transitions to `ManoeuvreNegotiate` if the
      request is accepted.
   */
   'TriggerDeltaNegotiation',
   /* 
      Outcome of negotiation has determined that no avoidance manoeuvre is required.
      Transitions to `ManoeuvreNotRequired`.
   */
   'ManoeuvreNotRequired',
   /* 
      Manoeuvre assignment resulting from the manoeuvre negotiation. This will assign the
      meneuver to one of the teams involved.
   */
   'ManoeuvreAssigned',
   /* 
      Indicates that manoeuvre plans have been uploaded. Transitions to `ManoeuvrePlanned`. 
   */
   'ManoeuvrePlanned',
   /* 
      Indicate that post-trajectory screening of the manoeuvre plans has been performed. 
      Transitions to `ManoeuvreScreened` 
   */
   'ManoeuvreScreened',
   /* 
      Indicates that the manoeuvre required re-planning. Transitions back to
      `ManoeuvreRequired` without changing the manoeuvre responsibility. 
   */
   'ManoeuvreChange', 
   /* 
      Indicates that the manoeuvre has been approved by the O/O (with or
      without trajectory screening by a third party). Transitions to `ManoeuvreSigned`. 
   */
   'ManoeuvreSigned', 
   /* 
      Indicates that the manoeuvre has been uplinked for execution.
      Transitions to `ManoeuvreUplinked`. 
   */
   'ManoeuvreUplinked',
   /* 
      Indicates that the manoeuvre has failed. Transitions to `ManoeuvreFailed`. 
   */
   'ManoeuvreFailed',
   /* 
      Indicates a user has sent a message to the counterparty. Status doesn't change. 
   */
   'MessageSent',
   /* 
      Indicates new data has arrived i.e. CDM. Status doesn't change. 
   */
   'NewCCSDSData'
);

CREATE TABLE negotiations (
    id SERIAL PRIMARY KEY,
    cdm_id INT NOT NULL,
    object1_id INT NOT NULL,
    object2_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
   CONSTRAINT fk_registered_object1
      FOREIGN KEY(object1_id) 
	  REFERENCES registered_objects(id)
      ON DELETE CASCADE,
   CONSTRAINT fk_registered_object2
      FOREIGN KEY(object2_id) 
	  REFERENCES registered_objects(id)
      ON DELETE CASCADE,
   CONSTRAINT fk_cdm_id
      FOREIGN KEY(cdm_id) 
	  REFERENCES conjunctions(id)
);

CREATE TABLE time_line (
    id SERIAL PRIMARY KEY,
    negotiation_id INT NOT NULL,
    initiator_id INT,
    subject VARCHAR NOT NULL,
    detail VARCHAR NOT NULL,
    action negotiation_action NOT NULL,
    resulting_status negotiation_status NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
   CONSTRAINT fk_negotiations
      FOREIGN KEY(negotiation_id) 
	  REFERENCES negotiations(id)
      ON DELETE CASCADE
);

-- Give readonly and application access to the database
GRANT SELECT, INSERT, UPDATE, DELETE ON negotiations TO trace_application;
GRANT USAGE, SELECT ON negotiations_id_seq TO trace_application;
GRANT SELECT ON negotiations TO trace_readonly;
GRANT SELECT ON negotiations_id_seq TO trace_readonly;

GRANT SELECT, INSERT ON time_line TO trace_application;
GRANT USAGE, SELECT ON time_line_id_seq TO trace_application;
GRANT SELECT ON time_line TO trace_readonly;
GRANT SELECT ON time_line_id_seq TO trace_readonly;


-- migrate:down

DROP TABLE time_line;
DROP TABLE negotiations;
DROP TYPE negotiation_status;
DROP TYPE negotiation_action;