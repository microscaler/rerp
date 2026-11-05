Feature: Branching steps - if/else, split, goal (jump to) and go to
  The graph is a tree with explicit joins: a branch either ends, or rejoins with a GOTO. A GOAL step is
  a labelled condition contacts can jump to from elsewhere; a GOTO moves a contact unconditionally.

  Background:
    Given an ACTIVE workflow "Free trial nurture" with these steps (the walked recipe):
      | id  | kind    | detail                                                              |
      | e1  | ACTION  | SEND_MESSAGE "Welcome"                                              |
      | w1  | WAIT    | 2 days                                                              |
      | e2  | ACTION  | SEND_MESSAGE "First steps"                                          |
      | w2  | WAIT    | 2 days                                                              |
      | c1  | IF_ELSE | tag "Has used Advanced Features" exists; yes -> e3, no -> e4        |
      | e3  | ACTION  | SEND_MESSAGE "Advanced tips" -> w3                                  |
      | w3  | WAIT    | 2 days -> j                                                         |
      | e4  | ACTION  | SEND_MESSAGE "Deep dive of advanced features" -> g1                 |
      | g1  | GOTO    | -> j                                                                |
      | j   | ACTION  | SEND_MESSAGE "Use cases" -> ...                                     |
      | c2  | IF_ELSE | tag "customer" exists; yes -> END, no -> t1                         |
      | t1  | ACTION  | ADD_TAG "Expired Trial" -> END                                      |

  Scenario: If/else takes the yes branch when the conditions hold at evaluation time
    Given "ana@example.com" has tag "Has used Advanced Features"
    When her enrolment reaches "c1"
    Then it moves to "e3"
    And the step record for "c1" shows branch YES

  Scenario: If/else takes the no branch otherwise, and conditions are evaluated when the contact arrives, not when it was enrolled
    Given "ben@example.com" had no tags when enrolled
    And tag "Has used Advanced Features" was added to him while he waited at "w2"
    When his enrolment reaches "c1"
    Then it moves to "e3"

  Scenario: A missing branch ends the enrolment
    Given "c2" has no no_step_id
    And "cat@example.com" has no tag "customer"
    When her enrolment reaches "c2"
    Then the enrolment ends with status COMPLETED

  Scenario: Go to rejoins the other branch and is terminal for its own
    Given "dan@example.com" has no tag "Has used Advanced Features"
    When his enrolment reaches "c1"
    Then it moves to "e4", then to "g1", then to "j"
    And "g1" has no next_step_id of its own

  Scenario: Go to may point upwards, which makes a loop; a loop without a wait is rejected at validation
    Given "g1" is changed to point at "e1"
    When the steps are validated
    Then validation reports CYCLE_WITHOUT_WAIT on "g1"
    Given "g1" is changed to point at "w1"
    When the steps are validated
    Then validation passes (the loop passes through a wait)

  Scenario Outline: Split - even and percentage paths
    Given a workflow whose entry step is SPLIT with mode <mode> and branches <branches>
    When 1000 contacts are enrolled
    Then the share of contacts on each path is <shares> within 3 points

    Examples:
      | mode    | branches                                   | shares         |
      | EVEN    | A, B                                       | 50 / 50        |
      | EVEN    | A, B, C                                    | 33 / 33 / 33   |
      | PERCENT | A 70, B 30                                 | 70 / 30        |

  Scenario: Split - a contact is assigned once and stays on its path through re-entry of the step
    Given a SPLIT step with mode EVEN and branches A, B
    When "ana@example.com" is assigned to path B
    And a GOTO sends her back through the split
    Then she is assigned to path B again

  Scenario: Split test picks a winner after the deciding count and sends later contacts down it
    Given a SPLIT step with mode TEST, until_count 200, branches A -> "Email A", B -> "Email B"
    And winner_conditions {"all": [{"engagement": "CLICKED", "template_id": "<branch message>"}]}
    When 200 contacts have been split and 18% of A and 31% of B met the conditions
    Then test.winner_key is "B"
    And the 201st contact is sent down path B
    And the 200 already split keep their paths

  Scenario: Split test by date decides at the date even with few contacts
    Given a SPLIT step with mode TEST, until_date 2026-12-01 and two branches
    When the date passes with 9 contacts split
    Then a winner is chosen from those 9 (ties go to A) and recorded

  Scenario: Split until sends everyone to A until the date, then everyone to B
    Given a SPLIT step with mode UNTIL, until_date 2026-12-01 and branches A, B
    When "ana@example.com" arrives on 2026-11-30
    Then she takes path A
    When "ben@example.com" arrives on 2026-12-01
    Then he takes path B

  Scenario: Goal (jump to) - a contact below the goal jumps to it when the conditions become true
    Given a workflow: "e1" -> "w1" (wait 30 days) -> "g" (GOAL tag "customer", scope BELOW) -> "thanks"
    And "ana@example.com" is waiting at "w1"
    When tag "customer" is added to her
    Then her enrolment moves to "g" and then to "thanks" without finishing the wait
    And a goal event is recorded for "g"

  Scenario: Goal scope BELOW ignores contacts above the goal
    Given a workflow: "g" (GOAL tag "customer", scope BELOW) -> "e1" -> "w1" -> "e2"
    And "ben@example.com" is waiting at "w1" (below the goal)
    When tag "customer" is added to him
    Then nothing happens to his enrolment

  Scenario: Goal scope ANYWHERE pulls contacts from any step, including above
    Given the same workflow with the goal's scope ANYWHERE
    And "ben@example.com" is waiting at "w1"
    When tag "customer" is added to him
    Then his enrolment moves to "g"

  Scenario: A contact reaching a goal in sequence without meeting it continues (if_not_met CONTINUE)
    Given "g" has if_not_met CONTINUE
    When "cat@example.com" reaches "g" in sequence without tag "customer"
    Then she moves to the step after "g"
    And the step record for "g" shows outcome SKIPPED

  Scenario: ... or waits there until it does (if_not_met WAIT)
    Given "g" has if_not_met WAIT
    When "cat@example.com" reaches "g" in sequence without tag "customer"
    Then she stays at "g"
    When tag "customer" is added
    Then she moves on, and the step record shows outcome ACHIEVED

  Scenario: Goal outcomes are themselves triggers for other workflows
    Given another workflow with trigger GOAL_REACHED {"workflow_id": "<this>", "goal_step_id": "g", "outcome": "ACHIEVED"}
    When "ana@example.com" achieves "g"
    Then she is enrolled in the other workflow
