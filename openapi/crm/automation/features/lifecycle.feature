Feature: Workflow lifecycle - readiness, activation, exit rules, pause and stop
  A workflow is DRAFT until it is activated. Activation is refused while the graph is not ready: a
  message step with no designed template, a sender without a postal address, an unbound recipe
  parameter. Exit rules end enrolments early.

  Background:
    Given a DRAFT workflow "Re-engagement" with trigger TAG_ADDED {"tag": "Disengaged"} and steps:
      | id | kind    | detail                                                                                   |
      | e1 | ACTION  | SEND_MESSAGE template <re-engagement email>                                               |
      | w1 | WAIT    | UNTIL_CONDITION clicked a link in <re-engagement email>, timeout 7 days                  |
      | c1 | IF_ELSE | clicked a link in <re-engagement email> OR opened it; yes -> END, no -> u1               |
      | u1 | ACTION  | UNSUBSCRIBE_LIST {"list_ids": ["<Newsletter>"]} -> END                                   |

  Scenario: A message step whose template is still a draft blocks activation
    Given the template <re-engagement email> has status DRAFT (no design)
    When a user activates the workflow
    Then the request fails with 422 and problems [{"code": "TEMPLATE_NOT_READY", "step_id": "e1"}]
    And the workflow stays DRAFT
    And GET /workflows/{id}/steps/validate lists the same problem (the builder shows it beside the step)

  Scenario: A sender without a postal address blocks activation of a workflow that sends marketing email
    Given the template's sender has no postal_address_id and the workspace has no default
    When a user activates the workflow
    Then the request fails with 422 and problems [{"code": "NO_POSTAL_ADDRESS", "step_id": "e1"}]

  Scenario: An unbound recipe parameter blocks activation
    Given the workflow was instantiated from a recipe and step "w1"'s template parameter is unbound
    When a user activates the workflow
    Then the request fails with 422 and problems [{"code": "UNBOUND_PARAMETER", "step_id": "w1"}]

  Scenario: A ready workflow activates and starts listening to its triggers
    Given every template is ready and every parameter bound
    When a user activates the workflow
    Then its status is ACTIVE
    And tag "Disengaged" added to "ana@example.com" enrols her

  Scenario: Exit on unsubscribe ends the enrolment when the contact unsubscribes from a message the workflow sent
    Given the workflow is ACTIVE with exit_rules {"on_unsubscribe": true}
    And "ana@example.com" is waiting at "w1" after receiving <re-engagement email>
    When she unsubscribes through that email's link
    Then her enrolment ends with status EXITED and exit_reason UNSUBSCRIBED
    And no further step runs for her

  Scenario: Without exit on unsubscribe the contact continues, but marketing sends to her are suppressed
    Given the workflow is ACTIVE with exit_rules {"on_unsubscribe": false}
    And "ana@example.com" is at "w1" and unsubscribes from the list
    When the wait times out and c1 takes the no branch to u1 and then to a later SEND_MESSAGE
    Then the enrolment continues to the send step
    And the send is recorded as SUPPRESSED with reason NOT_SUBSCRIBED

  Scenario: Unsubscribe scope decides which lists the message's unsubscribe link acts on
    Given the workflow is ACTIVE with unsubscribe_scope TRIGGER_LISTS and its only list reference is "Newsletter"
    And "ana@example.com" is on "Newsletter" and "Customers"
    When she unsubscribes through the workflow's email
    Then she is unsubscribed from "Newsletter" only
    Given instead unsubscribe_scope ALL_LISTS
    Then she is unsubscribed from both lists

  Scenario: Exit on conversion ends the enrolment when a tracked conversion is recorded
    Given the workflow is ACTIVE with tracking {"conversion_id": "<Trial signup>"} and exit_rules {"on_conversion": true}
    And "ana@example.com" is waiting at "w1"
    When conversion <Trial signup> is recorded for her
    Then her enrolment ends with status EXITED and exit_reason CONVERTED
    And the conversion is attributed to this workflow in the conversions report

  Scenario: Pausing stops the clock and the sends; resuming picks up where each enrolment was
    Given the workflow is ACTIVE and "ana@example.com" is at "w1" with 3 days of timeout left
    When the workflow is paused for 10 days and then resumed
    Then she is still at "w1" with 3 days of timeout left
    And nothing was sent to her while paused

  Scenario: Stopping another workflow ends that contact's enrolment there
    Given an ACTIVE workflow "Expired trial" in which "ana@example.com" is enrolled
    And this workflow has an ACTION step STOP_WORKFLOW {"workflow_id": "<Expired trial>"}
    When her enrolment here reaches that step
    Then her enrolment in "Expired trial" ends with status EXITED and exit_reason STOPPED_BY_WORKFLOW
    And her enrolment here continues

  Scenario: Starting another workflow enrols the contact there with trigger STARTED_BY_WORKFLOW
    Given this workflow has an ACTION step START_WORKFLOW {"workflow_id": "<Expired trial>"}
    When her enrolment reaches it
    Then an enrolment exists for her in "Expired trial" with trigger STARTED_BY_WORKFLOW
    And "Expired trial"'s reentry rule applies to that enrolment

  Scenario: Deleting a workflow with active enrolments is refused; archiving ends them
    Given the workflow is ACTIVE with 12 active enrolments
    When a user deletes it
    Then the request fails with 409 ACTIVE_ENROLMENTS
    When a user archives it
    Then its status is ARCHIVED and the 12 enrolments end with exit_reason WORKFLOW_ARCHIVED

  Scenario: Editing the graph of an active workflow keeps enrolments on steps that still exist
    Given the workflow is ACTIVE and "ana@example.com" is at "w1"
    When step "c1" is replaced by a new IF_ELSE "c2" and "w1".next_step_id is "c2"
    Then she is still at "w1" and will continue to "c2"
    When step "w1" itself is deleted
    Then her enrolment is moved to the deleted step's next step and the step record notes STEP_REMOVED

  Scenario: Each step keeps counts the builder shows
    Given the workflow is ACTIVE and 40 contacts have passed "e1", 25 are waiting at "w1"
    Then GET /workflows/{id}/map shows e1 {passed: 40, waiting: 0} and w1 {passed: 15, waiting: 25}
    And SEND_MESSAGE steps also show sent, open_rate and click_rate for their template
