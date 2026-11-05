Feature: Enrolling contacts in a workflow
  A workflow starts a contact's enrolment when one of its start triggers fires, when a user adds the
  contact by hand, or when another workflow starts it. How often a contact may enter is the workflow's
  reentry rule; each trigger also says whether it fires once per contact or every time.

  Background:
    Given a workspace with a list "Newsletter" and a tag "Expired Trial"
    And an ACTIVE workflow "Free trial nurture" whose triggers are:
      | type               | config                | runs |
      | SUBSCRIBED_TO_LIST | {"list_id": "<Newsletter>"} | ONCE |

  Scenario: A trigger event enrols the contact at the entry step
    When contact "ana@example.com" subscribes to "Newsletter"
    Then an enrolment exists for "ana@example.com" in "Free trial nurture"
    And its current step is the workflow's entry step
    And the enrolment's trigger is recorded as SUBSCRIBED_TO_LIST on "Newsletter"

  Scenario: "Any list" triggers fire for every list
    Given the trigger's config is {} instead
    When contact "ana@example.com" subscribes to "Customers"
    Then an enrolment exists for "ana@example.com" in "Free trial nurture"

  Scenario: A trigger that runs once never fires twice for the same contact
    Given "ana@example.com" was already enrolled by this trigger and has exited
    When contact "ana@example.com" unsubscribes and subscribes to "Newsletter" again
    Then no new enrolment is created

  Scenario: A trigger that runs multiple times fires again after the contact has exited
    Given the trigger's runs is MULTIPLE and the workflow's reentry is MULTIPLE
    And "ana@example.com" was already enrolled by this trigger and has exited
    When contact "ana@example.com" subscribes to "Newsletter" again
    Then a second enrolment exists for "ana@example.com"

  Scenario Outline: The reentry rule decides what a firing trigger does while an enrolment is active
    Given the trigger's runs is MULTIPLE and the workflow's reentry is <reentry>
    And "ana@example.com" has an ACTIVE enrolment waiting at step "Wait 2 days"
    When contact "ana@example.com" subscribes to "Newsletter" again
    Then <outcome>

    Examples:
      | reentry    | outcome                                                               |
      | ONCE       | nothing changes                                                       |
      | MULTIPLE   | nothing changes, because the active enrolment has not exited          |
      | CONCURRENT | a second ACTIVE enrolment starts at the entry step alongside the first |
      | RESTART    | the active enrolment is ended with status RESTARTED and a new one starts |

  Scenario: A trigger's segment filter gates entry through that trigger only
    Given the trigger has segment {"all": [{"field": "plan", "op": "EQ", "value": "trial"}]}
    When contact "ana@example.com" with plan "pro" subscribes to "Newsletter"
    Then no enrolment is created
    When contact "ben@example.com" with plan "trial" subscribes to "Newsletter"
    Then an enrolment exists for "ben@example.com"

  Scenario: Several start triggers are alternatives, not a conjunction
    Given the workflow also has trigger TAG_ADDED {"tag": "trial"} runs ONCE
    When tag "trial" is added to "cat@example.com" who is on no list
    Then an enrolment exists for "cat@example.com"
    And its trigger is recorded as TAG_ADDED "trial"

  Scenario: Adding specific contacts by hand enrols them regardless of triggers
    When a user posts to /workflows/{id}/enrol with contact_ids ["dan@example.com"]
    Then an enrolment exists for "dan@example.com" with trigger MANUAL
    And the response reports enrolled 1, skipped 0

  Scenario: Bulk-adding a segment respects the reentry rule and reports the skips
    Given "ana@example.com" has an ACTIVE enrolment and the workflow's reentry is ONCE
    And a saved segment "Trialists" matches "ana@example.com", "eve@example.com" and "fay@example.com"
    When a user posts to /workflows/{id}/enrol with segment_id "<Trialists>"
    Then enrolments exist for "eve@example.com" and "fay@example.com" with trigger SEGMENT
    And the response reports enrolled 2, skipped 1 with reason ALREADY_ENROLLED

  Scenario: An inactive workflow enrols nobody
    Given the workflow's status is DRAFT
    When contact "ana@example.com" subscribes to "Newsletter"
    Then no enrolment is created
    And the trigger is not recorded as fired

  Scenario: A paused workflow keeps its enrolments where they are and enrols nobody
    Given "ana@example.com" has an ACTIVE enrolment waiting at step "Wait 2 days"
    When the workflow is paused
    And the wait elapses
    Then the enrolment is still at "Wait 2 days"
    When the workflow is resumed
    Then the enrolment moves on to the next step
