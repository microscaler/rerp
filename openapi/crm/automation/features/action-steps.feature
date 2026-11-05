Feature: Action steps
  An ACTION step does one thing and continues. Sends go through crm/outreach and respect consent;
  everything else mutates the contact, deal or account, or talks to staff or another system.

  Background:
    Given an ACTIVE workflow and contact "ana@example.com" enrolled, arriving at the step under test

  Scenario: SEND_MESSAGE sends the automation template to a consenting contact and records the send
    Given the step is SEND_MESSAGE {"template_id": "<Welcome>"}
    And "ana@example.com" is subscribed to a list the template's sender may mail
    Then a send exists for her with template <Welcome>, source WORKFLOW, workflow_id and step_id set
    And the step's counters show sent 1
    And the enrolment moves on without waiting for delivery

  Scenario: SEND_MESSAGE to a contact with no list consent is suppressed, not sent, and the enrolment continues
    Given "ana@example.com" is on no list
    Then the send is recorded as SUPPRESSED with reason NOT_SUBSCRIBED
    And the enrolment moves on

  Scenario: SEND_MESSAGE with a draft template at run time is skipped with an error record
    Given the template was reverted to DRAFT after activation
    Then the step record shows outcome ERROR "TEMPLATE_NOT_READY"
    And the enrolment moves on

  Scenario: INTERNAL_NOTIFICATION emails staff without consent and with a contact link
    Given the step is INTERNAL_NOTIFICATION {"subject": "Hot lead: %FIRSTNAME%", "to": ["sales@example.com"], "body": "...", "include_contact_link": true}
    Then an email is sent to sales@example.com with the merged subject and a link to /contacts/{id}
    And no consent check applies and nothing is recorded on the contact's engagement timeline
    And more than 5 recipients is rejected at validation

  Scenario: SEND_1TO1_EMAIL goes out as plain text from the user's identity with their reply-to
    Given the step is SEND_1TO1_EMAIL {"from_user_id": "<rep>", "subject": "Quick question", "text": "..."}
    Then a send exists with kind ONE_TO_ONE, from the rep's sender identity, reply_to the rep's address, no unsubscribe footer
    And it counts in the contact's timeline but not in campaign reports

  Scenario: CREATE_TASK creates a task on the contact, or on their open deal
    Given the step is CREATE_TASK {"title": "Call about trial", "task_type_id": "<Call>", "assignee": "OWNER", "due_after_seconds": 172800}
    Then a crm/tasks task exists for "ana@example.com", type Call, owner = the contact's owner, due in 2 days
    Given instead {"deal": true, "pipeline_id": "<Sales>"}
    And she has an open deal in Sales
    Then the task is attached to that deal
    And without an open deal the step records outcome SKIPPED "NO_OPEN_DEAL"

  Scenario: COMPLETE_TASK closes the newest open task of the type with the given outcome
    Given she has an open Call task
    And the step is COMPLETE_TASK {"task_type_id": "<Call>", "outcome": "Reached"}
    Then that task is COMPLETED with outcome Reached, completed_by = workflow
    And the TASK_COMPLETED trigger fires for workflows listening on it

  Scenario Outline: Deal actions apply to the contact's open deal in the pipeline
    Given she has an open deal "Trial -> paid" in Sales
    And the step is <action>
    Then <effect>

    Examples:
      | action                                                   | effect                                               |
      | UPDATE_DEAL_STAGE {"pipeline_id": "<Sales>", "stage_id": "<Proposal>"} | the deal is in Proposal and DEAL_STAGE_CHANGE fires |
      | UPDATE_DEAL_OWNER {"user_id": "<rep>"}                   | the deal's owner is the rep                          |
      | UPDATE_DEAL_VALUE {"delta": 500}                         | the deal's value is 500 higher                       |
      | UPDATE_DEAL_STATUS {"status": "WON"}                     | the deal is WON and DEAL_WON fires                   |
      | ADD_DEAL_NOTE {"text": "nurtured by workflow"}           | the deal has the note, author = workflow             |
      | ADJUST_DEAL_SCORE {"score_id": "<Deal health>", "points": -10} | the deal's score is 10 lower                   |

  Scenario: Deal actions on a contact with no open deal are skipped
    Given she has no open deal
    And the step is UPDATE_DEAL_STAGE {"pipeline_id": "<Sales>", "stage_id": "<Proposal>"}
    Then the step record shows outcome SKIPPED "NO_OPEN_DEAL" and the enrolment continues

  Scenario: CREATE_DEAL opens a deal and the ENTERED_PIPELINE trigger fires
    Given the step is CREATE_DEAL {"pipeline_id": "<Sales>", "stage_id": "<New>", "title": "%FIRSTNAME% - trial", "owner": "OWNER"}
    Then a deal "Ana - trial" exists in Sales / New owned by her owner, with her as Primary contact
    And ENTERED_PIPELINE and DEAL_CREATED fire

  Scenario: ADD_TO_ACCOUNT links the contact to a company by email domain
    Given the step is ADD_TO_ACCOUNT {"account_id": "FROM_EMAIL_DOMAIN"}
    And a company with domain "example.com" exists
    Then "ana@example.com" is linked to that company
    And with no matching company a new one named "example.com" is created

  Scenario: CONVERT records a conversion once per contact
    Given the step is CONVERT {"conversion_id": "<Trial signup>"}
    Then a conversion event exists for her attributed to this workflow and step
    When she reaches the same step again in a later enrolment
    Then no second event is recorded (conversions are once per contact) and the step records outcome SKIPPED "ALREADY_CONVERTED"

  Scenario: ARCHIVE_CONTACT removes the contact from sends and segments but keeps history
    Given the step is ARCHIVE_CONTACT {}
    Then her status is ARCHIVED, she is excluded from every audience, and her enrolment ends with exit_reason ARCHIVED

  Scenario: FORMAT_DATA operations
    Given she has custom fields seats "12" and renewal_date 2026-12-01
    And the step is <op>
    Then <result>

    Examples:
      | op                                                                                     | result                                  |
      | FORMAT_DATA {"op": "MATH", "source_field": "seats", "operator": "MULTIPLY", "operand": 2, "target_field": "seats"} | seats is "24" |
      | FORMAT_DATA {"op": "DATE_ADJUST", "source_field": "renewal_date", "amount": -14, "unit": "DAYS", "target_field": "renewal_reminder"} | renewal_reminder is 2026-11-17 |
      | FORMAT_DATA {"op": "COPY", "source_field": "email", "target_field": "login"}            | login is "ana@example.com"              |
      | FORMAT_DATA {"op": "CAPITALIZE", "source_field": "first_name", "case": "TITLE"}         | first_name "ana" becomes "Ana"          |
      | FORMAT_DATA {"op": "EXTRACT", "source_field": "notes", "extract": "PHONE", "target_field": "phone"} | phone holds the first number found |

  Scenario: CALL_WEBHOOK posts the contact and continues regardless of the response
    Given the step is CALL_WEBHOOK {"url": "https://hooks.example.com/x", "secret_id": "<s>"}
    Then a POST is made with the contact, workflow_id, step_id and an HMAC signature header
    And a 500 response is retried 3 times with backoff and then recorded as outcome ERROR
    And the enrolment moves on either way

  Scenario: Every action records a step record the activity log shows
    Then the enrolment's step records list each step with entered_at, left_at, outcome and, for sends, the send id
