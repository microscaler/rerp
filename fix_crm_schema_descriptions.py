"""
Item 5: Add description to ALL schemas across 11 CRM specs.

Previously schemas had descriptions but many were missing or generic.
This adds proper descriptions for every schema definition.
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Schema descriptions by entity type
SCHEMA_DESCRIPTIONS = {
    'Lead': 'A potential customer who has shown interest in our products or services',
    'Contact': 'An individual person associated with an account or lead',
    'Account': 'A company or organization that is a customer or prospect',
    'Stage': 'A pipeline stage representing a step in the sales process',
    'Team': 'A group of sales representatives organized by region or product line',
    'Workflow': 'An automation workflow that defines business logic for lead processing',
    'Rule': 'A conditional rule that triggers actions when certain criteria are met',
    'Trigger': 'An event-based trigger that initiates workflow execution',
    'ScoringFrequency': 'Configuration for how often lead scores should be recalculated',
    'EnrichmentResult': 'Enriched data obtained from external sources for a lead',
    'ChatSession': 'A live chat session between a visitor and an agent',
    'ChatMessage': 'A single message within a chat session',
    'Agent': 'A customer support agent who handles live chat sessions',
    'Campaign': 'A marketing campaign used to generate leads and track attribution',
    'Subscription': 'A customer subscription to a product or service plan',
    'Goal': 'A target metric for sales teams or individuals to achieve',
    'CustomField': 'A custom field definition for extending entity schemas',
    'Webhook': 'A webhook endpoint for receiving event notifications',
    'ApiKey': 'An API key used for authentication and authorization',
    'Integration': 'A third-party integration configuration',
    'Report': 'A saved report definition with query parameters',
    'Summary': 'A summary object containing aggregated metrics and statistics',
    'Analytics': 'Analytics data for tracking marketing and visitor metrics',
    'PipelineSummary': 'A summary of pipeline metrics including stage breakdown and weighted values',
    'LeadScore': 'A lead score with its calculated value and level',
    'LeadScoreSummary': 'Aggregated lead score distribution across hot/warm/cold levels',
    'LeadScoreRanked': 'A ranked lead with its score and position in the ranking',
    'ScoreExplanation': 'A detailed explanation of how a lead score was calculated',
    'ScoreThresholds': 'Configuration for score thresholds defining hot/warm/cold levels',
    'ConversionResult': 'Result of converting a lead to a contact or account',
    'DuplicateDetectionResult': 'Result of duplicate detection with matched records',
    'MergeResult': 'Result of merging multiple records together',
    'WorkflowExecutionResult': 'Result of executing a workflow including triggered rules',
    'RuleTestResult': 'Result of testing a rule configuration',
    'ScheduledAction': 'A scheduled action with its execution parameters',
    'ActionUpcoming': 'An upcoming scheduled action with repeat interval',
    'VerificationResult': 'Result of verifying an email or phone number',
    'EnrichmentLookupResult': 'Result of enriching data from a website or email lookup',
    'MiningSearchResult': 'Result of searching for leads matching ICP criteria',
    'ICPMatchResult': 'Result of finding leads matching ideal customer profile',
    'AnalyticsSummary': 'Aggregated analytics summary with visitor metrics',
    'ConversionRate': 'Conversion rate analytics with breakdown by source',
    'FunnelData': 'Sales funnel data showing conversion between stages',
    'LeadSourceReport': 'Lead source attribution report with counts by channel',
    'RepPerformance': 'Sales representative performance metrics',
    'TeamPerformance': 'Team-level performance metrics',
    'WinLossAnalysis': 'Win/loss analysis with breakdown by reason',
    'TimeToCloseAnalysis': 'Time-to-close analysis by pipeline stage',
    'ForecastEntry': 'A single forecast entry for a period',
    'ForecastAccuracyEntry': 'Forecast accuracy comparison (forecasted vs actual)',
    'LeaderboardEntry': 'A leaderboard entry with ranking and metrics',
    'AtRiskCustomer': 'An at-risk customer with churn probability and risk factors',
    'HealthScore': 'Customer health score with contributing factors',
    'RenewalSummary': 'Renewal analytics with upcoming and completed renewals',
    'MRRSummary': 'Monthly Recurring Revenue summary with churn and expansion',
    'ChurnSummary': 'Churn rate summary with subscription counts',
    'BadgeResult': 'An award badge with its criteria and description',
    'QueueItem': 'An unassigned lead in the queue ready for assignment',
    'AgentMetrics': 'Agent performance metrics including response times and satisfaction',
    'AgentStatus': 'Current agent status and availability',
    'AgentQueueItem': 'A visitor waiting in the agent queue',
    'AuditLogEntry': 'An audit log entry recording a system action',
    'WebhookDeliveryLog': 'A log entry for webhook delivery attempts',
    'CampaignLeadResult': 'A lead result attributed to a campaign',
    'EventRegistration': 'An event registration record',
    'FormSubmissionResult': 'Result of a marketing form submission',
    'SurveyResponse': 'A survey response with answers and feedback',
    'TeamMember': 'A team member with their role and details',
    'TeamMembers': 'A collection of team members in a team',
    'VisitorTracking': 'A visitor tracking record with page and session data',
    'WebForm': 'A marketing form definition with its fields',
    'WeightedPipeline': 'Pipeline data with weighted values by stage',
    'PaginatedLead': 'A paginated list of Lead records',
    'PaginatedContact': 'A paginated list of Contact records',
    'PaginatedAccount': 'A paginated list of Account records',
    'PaginatedStage': 'A paginated list of Stage records',
    'PaginatedTeam': 'A paginated list of Team records',
    'PaginatedWorkflow': 'A paginated list of Workflow records',
    'PaginatedRule': 'A paginated list of Rule records',
    'PaginatedTrigger': 'A paginated list of Trigger records',
    'PaginatedChatSession': 'A paginated list of ChatSession records',
    'PaginatedChatMessage': 'A paginated list of ChatMessage records',
    'PaginatedAgent': 'A paginated list of Agent records',
    'PaginatedCampaign': 'A paginated list of Campaign records',
    'PaginatedSubscription': 'A paginated list of Subscription records',
    'PaginatedGoal': 'A paginated list of Goal records',
    'PaginatedCustomField': 'A paginated list of CustomField records',
    'PaginatedWebhook': 'A paginated list of Webhook records',
    'PaginatedApiKey': 'A paginated list of ApiKey records',
    'PaginatedIntegration': 'A paginated list of Integration records',
    'PaginatedReport': 'A paginated list of Report records',
    'PaginatedUtMCampaign': 'A paginated list of UTMCampaign records',
    'PaginatedUtMMedium': 'A paginated list of UTMMedium records',
    'PaginatedUtMSource': 'A paginated list of UTMSource records',
    'ErrorResponse': 'An error response with code, message, and optional details',
    'PaginatedResponse': 'A generic paginated response wrapper with items and pagination metadata',
}


def process_spec(spec_path):
    """Process a single OpenAPI spec file and add descriptions to schemas."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    schemas = data.get('components', {}).get('schemas', {})
    
    for schema_name, schema_def in schemas.items():
        if not isinstance(schema_def, dict):
            continue
        
        # Skip if already has a description
        if schema_def.get('description'):
            continue
        
        # Skip if it's a wrapper type (allOf, anyOf, oneOf) without properties
        if 'allOf' in schema_def or 'anyOf' in schema_def or 'oneOf' in schema_def:
            # Check if it's a PaginatedResponse or ErrorResponse
            if schema_name in ['PaginatedResponse', 'ErrorResponse']:
                desc = SCHEMA_DESCRIPTIONS.get(schema_name, f'{schema_name} schema')
                schema_def['description'] = desc
                modified += 1
            continue
        
        # Get description from template
        if schema_name in SCHEMA_DESCRIPTIONS:
            schema_def['description'] = SCHEMA_DESCRIPTIONS[schema_name]
            modified += 1
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    return modified


# Process all specs
total_fixed = 0
for item in sorted(os.listdir(crm_base)):
    spec_path = os.path.join(crm_base, item, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    fixed = process_spec(spec_path)
    if fixed:
        print(f"  {item}: {fixed} schema descriptions added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} schema descriptions added")
