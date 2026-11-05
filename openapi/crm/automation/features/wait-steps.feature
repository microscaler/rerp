Feature: Wait steps
  A WAIT step holds an enrolment until a time or a condition. It never sends anything itself; when it
  releases, the enrolment moves to next_step_id (or ends if there is none).

  Background:
    Given an ACTIVE workflow with entry step "Email A" -> step "W" (WAIT) -> step "Email B"
    And contact "ana@example.com" enrolled and sitting at "W" since 2026-11-05T10:00:00Z

  Scenario: Waiting for a set period
    Given "W" is {"mode": "DURATION", "seconds": 172800}
    When the clock reaches 2026-11-07T09:59:59Z
    Then the enrolment is still at "W"
    When the clock reaches 2026-11-07T10:00:00Z
    Then the enrolment is at "Email B"
    And the step record for "W" shows entered_at 2026-11-05T10:00:00Z and left_at 2026-11-07T10:00:00Z

  Scenario: Waiting until a specific instant
    Given "W" is {"mode": "UNTIL_DATETIME", "until": "2026-11-24T08:00:00Z"}
    Then the enrolment stays at "W" until 2026-11-24T08:00:00Z and then moves on

  Scenario: Waiting until a day-and-time pattern, in the contact's timezone
    Given "W" is {"mode": "UNTIL_DATETIME", "until_pattern": {"day": 24, "time": "08:00", "timezone": "CONTACT"}}
    And "ana@example.com" has timezone "America/New_York"
    Then the enrolment releases at the next 24th of a month at 08:00 America/New_York

  Scenario: A pattern whose month, day and year are all open means "the next time of day"
    Given "W" is {"mode": "UNTIL_DATETIME", "until_pattern": {"time": "08:00", "timezone": "WORKSPACE"}}
    And the workspace timezone is Europe/Sofia
    Then the enrolment releases at 2026-11-06T08:00:00 Europe/Sofia

  Scenario: An instant already in the past releases immediately
    Given "W" is {"mode": "UNTIL_DATETIME", "until": "2026-01-01T00:00:00Z"}
    Then the enrolment is at "Email B" as soon as it reaches "W"

  Scenario Outline: Waiting for a contact date field, on, before or after it
    Given "ana@example.com" has renewal_date 2026-12-01
    And "W" is {"mode": "UNTIL_DATE_FIELD", "date_field": "renewal_date", "relation": "<relation>", "offset_seconds": 259200}
    Then the enrolment releases on <release>

    Examples:
      | relation | release    |
      | ON       | 2026-12-01 |
      | BEFORE   | 2026-11-28 |
      | AFTER    | 2026-12-04 |

  Scenario: A date field that has already passed, by default, lets the contact continue at once
    Given "ana@example.com" has renewal_date 2026-10-01
    And "W" is {"mode": "UNTIL_DATE_FIELD", "date_field": "renewal_date", "relation": "ON"}
    Then the enrolment is at "Email B" as soon as it reaches "W"

  Scenario: A date field that has already passed can exit the contact instead
    Given "ana@example.com" has renewal_date 2026-10-01
    And "W" is {"mode": "UNTIL_DATE_FIELD", "date_field": "renewal_date", "relation": "ON", "if_date_passed": "EXIT"}
    Then the enrolment ends with status EXITED and exit_reason DATE_PASSED

  Scenario: An empty date field holds the contact until it is filled
    Given "ana@example.com" has no renewal_date
    And "W" is {"mode": "UNTIL_DATE_FIELD", "date_field": "renewal_date", "relation": "ON"}
    Then the enrolment stays at "W"
    When renewal_date is set to 2026-11-06
    Then the enrolment releases on 2026-11-06

  Scenario: Waiting until conditions are met
    Given "W" is {"mode": "UNTIL_CONDITION", "conditions": {"all": [{"engagement": "CLICKED", "template_id": "<Email A>"}]}}
    When "ana@example.com" clicks a link in "Email A"
    Then the enrolment is at "Email B" within one evaluation cycle

  Scenario: A conditional wait with a timeout continues anyway when it expires
    Given "W" is {"mode": "UNTIL_CONDITION", "conditions": {"all": [{"engagement": "CLICKED", "template_id": "<Email A>"}]}, "timeout_seconds": 604800}
    When seven days pass with no click
    Then the enrolment is at "Email B"
    And the step record for "W" shows outcome TIMED_OUT

  Scenario: A conditional wait without a timeout holds indefinitely
    Given "W" is {"mode": "UNTIL_CONDITION", "conditions": {"all": [{"tag": "customer"}]}}
    When a year passes
    Then the enrolment is still at "W" with status ACTIVE

  Scenario: Conditions already true on arrival release immediately
    Given "ana@example.com" has tag "customer"
    And "W" is {"mode": "UNTIL_CONDITION", "conditions": {"all": [{"tag": "customer"}]}}
    Then the enrolment is at "Email B" as soon as it reaches "W"

  Scenario: A send window delays the release to the contact's allowed hours
    Given "W" is {"mode": "DURATION", "seconds": 3600, "send_window": {"days": ["MON","TUE","WED","THU","FRI"], "from_hour": 9, "to_hour": 17}}
    And the duration would end on Saturday 2026-11-07T11:00:00 in the contact's timezone
    Then the enrolment releases on Monday 2026-11-09T09:00:00 in the contact's timezone

  Scenario: Editing a wait while contacts sit in it re-plans them from their entry time
    Given "W" is {"mode": "DURATION", "seconds": 172800}
    When "W" is changed to {"mode": "DURATION", "seconds": 86400}
    Then the enrolment releases at 2026-11-06T10:00:00Z
