Feature: Recipes - starter workflows with parameters
  A recipe is a workflow graph with placeholders (a list, a tag, a page URL, a message) and a note on
  each step explaining it. Instantiating one creates a DRAFT workflow; the placeholders become
  parameters to bind, and the workflow cannot activate until they are.

  Scenario: The library lists recipes by category with their trigger types and parameters
    When a user lists /recipes
    Then each recipe has id, name, category, description, trigger_types[], step_count, needs_templates
    And parameters[] such as [{"key": "list", "kind": "LIST", "label": "Select list", "required": true}]

  Scenario: Instantiating a recipe with bindings yields a DRAFT workflow with every reference resolved
    Given recipe "free-trial-nurture" with parameters list (LIST), advanced_tag (TAG), customer_tag (TAG), expired_tag (TAG)
    When a user posts /recipes/free-trial-nurture/instantiate with
      """
      {"name": "Free trial nurture", "bindings": {"list": "<Newsletter>", "advanced_tag": "Has used Advanced Features",
       "customer_tag": "customer", "expired_tag": "Expired Trial"}}
      """
    Then a DRAFT workflow "Free trial nurture" exists with triggers [SUBSCRIBED_TO_LIST {"list_id": "<Newsletter>"}]
    And its IF_ELSE steps test the bound tags
    And its 10 SEND_MESSAGE steps reference 10 new AUTOMATION templates in status DRAFT, titled from the recipe

  Scenario: Instantiating without bindings leaves parameters unbound and visible
    When a user posts /recipes/free-trial-nurture/instantiate with {}
    Then the workflow is created in DRAFT
    And GET /workflows/{id} lists unbound_parameters ["list", "advanced_tag", "customer_tag", "expired_tag"]
    And the step that uses an unbound parameter renders its label as "(select a tag)" rather than failing

  Scenario: The setup wizard is the ordered list of unbound parameters
    Given the workflow was instantiated with only "list" bound
    When a user gets /workflows/{id}/setup
    Then it returns steps [{"parameter": "advanced_tag", ...}, {"parameter": "customer_tag", ...}, {"parameter": "expired_tag", ...}] with "Step 1/3"
    When the user binds "advanced_tag" through PATCH /workflows/{id}/bindings
    Then /setup returns the remaining two

  Scenario: Notes travel with the recipe and can be edited
    When the recipe is instantiated
    Then step "c1" has notes [{"text": "This IF/ELSE is another example of ..."}, {"text": "In this example, if the contact has used ..."}, {"text": "If you want to build out reactive messaging ..."}]
    And the trigger has one note
    When a user adds a note to "c1"
    Then "c1" has four notes, the new one last, with author and created_at

  Scenario: A recipe can be imported from a URL and exported to one
    When a user posts /recipes/import with {"url": "https://.../recipe.json"}
    Then a recipe is added to the workspace's own category with source_url set
    When a user gets /workflows/{id}/export
    Then the response is a recipe document: the graph with references replaced by parameters, plus notes

  Scenario: Recipes reference nothing that belongs to a workspace
    Then no recipe in the library contains a list_id, tag, template_id, pipeline_id or stage_id literal
    And every such reference is a parameter

  Scenario: The walked recipes exist in the library
    Then /recipes includes "free-trial-nurture", "expired-trial-follow-up", "upgrade-incentive", "re-engagement-by-tag",
      "welcome-series", "abandoned-signup", "hot-lead-handoff"
