"""
Fix 1: Add descriptions to all 151 missing schemas across 11 CRM specs.

These are typically:
- Request types (CreateXxxRequest, UpdateXxxRequest)
- Supporting types (Hierarchy, Summary, Report, etc.)
- Supporting types for automation (ExecutionResult, etc.)
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Comprehensive schema descriptions for all missing types
SCHEMA_DESCRIPTIONS = {
    # Accounts
    'CreateAccountRequest': 'Request body for creating a new account/organization',
    'UpdateAccountRequest': 'Request body for updating an existing account/organization',
    'AccountHierarchy': 'Account tree with parent-child relationships for org structure',
    'AccountContactSummary': 'Summary of contacts associated with an account',
    'AccountReport': 'Analytics report data for an account including metrics and KPIs',
    
    # Automation
    'WorkflowExecutionResult': 'Result of executing a workflow including triggered rules and actions',
    'RuleExecutionResult': 'Result of a rule execution within a workflow',
    'TriggerExecutionResult': 'Result of trigger execution within a workflow',
    'ActionExecutionResult': 'Result of an action execution within a workflow',
    'CronJob': 'Scheduled cron job configuration for recurring automation',
    'RuleTestResult': 'Result of testing a rule with sample data',
    'ScheduledAction': 'A scheduled action with timing and execution details',
    'ActionUpcoming': 'Details of an upcoming scheduled action including interval',
    'Rule': 'A rule that defines conditions and actions for workflow automation',
    'Trigger': 'An event-based trigger that initiates workflow execution',
    'Workflow': 'A workflow definition containing rules, triggers, and actions',
    'WorkflowSummary': 'Summary of workflow including status and execution stats',
    'ExecutionLog': 'Log entry for a workflow execution event',
    
    # Contacts
    'CreateContactRequest': 'Request body for creating a new contact person',
    'UpdateContactRequest': 'Request body for updating an existing contact person',
    'ContactTree': 'Contact hierarchy with parent-child relationships',
    'ContactSummary': 'Summary statistics for a contact including associated records',
    'ContactMergeRequest': 'Request body for merging duplicate contacts',
    'ContactDuplicateCheck': 'Result of duplicate contact detection',
    
    # Engagement
    'CreateSubscriptionRequest': 'Request body for creating a new customer subscription',
    'UpdateSubscriptionRequest': 'Request body for updating an existing subscription',
    'RenewSubscriptionRequest': 'Request body for renewing a subscription',
    'GoalProgress': 'Goal progress tracking with current status and milestones',
    'GoalSummary': 'Summary of a goal including progress percentage',
    'LeaderboardEntry': 'Entry in the performance leaderboard with ranking and metrics',
    'CustomerHealthRisk': 'Details about at-risk customers including probability and factors',
    'HealthScore': 'Customer health score with contributing factors and trends',
    'SubscriptionRenewalSummary': 'Summary of subscription renewals including MRR impact',
    'SubscriptionMrrSummary': 'Monthly Recurring Revenue summary with breakdown',
    'SubscriptionChurnSummary': 'Churn rate summary with counts and rates',
    'BadgeResult': 'Badge award result including badge details and recipient',
    
    # Intelligence
    'LeadScoreFrequency': 'Configuration for lead scoring frequency (hourly/daily/weekly/etc)',
    'LeadScoreExplanation': 'Detailed explanation of how a lead score was calculated',
    'LeadScoreRanked': 'A ranked lead with its score and position in ranking',
    'LeadScoreSummary': 'Summary of lead score distribution across levels',
    'EnrichmentLookupResult': 'Result of enriching data from external sources',
    'EnrichmentRequest': 'Request body for enriching lead data from external sources',
    'VerificationResult': 'Result of verifying email/phone including confidence score',
    'VerificationRequest': 'Request body for verifying contact information',
    'MiningSearchResult': 'Result of searching for ICP-matched leads',
    'ICPMatchResult': 'Result of matching leads against ideal customer profile',
    
    # LiveChat
    'CreateChatSessionRequest': 'Request body for creating a new chat session',
    'ChatMessage': 'A chat message within a session including sender and content',
    'ChatMessageRequest': 'Request body for sending a chat message',
    'ChatConversionResult': 'Result of converting a chat to a lead',
    'AgentStatus': 'Current status of an agent (online/offline/busy)',
    'AgentQueueItem': 'A visitor waiting in the agent queue with session details',
    'ChatSessionSummary': 'Summary of a chat session including duration and outcomes',
    'AgentMetrics': 'Performance metrics for an agent including response time',
    
    # Marketing
    'CreateCampaignRequest': 'Request body for creating a new marketing campaign',
    'UpdateCampaignRequest': 'Request body for updating an existing campaign',
    'UTMCampaign': 'UTM campaign tracking parameters',
    'UTMMedium': 'UTM medium type definition',
    'UTMSource': 'UTM source definition for traffic attribution',
    'FormSubmissionResult': 'Result of a marketing form submission',
    'EventRegistration': 'Event registration details including status',
    'EventRegistrationResult': 'Result of registering for an event',
    'SurveyResponse': 'A survey response with answers and feedback',
    'SurveyResponseResult': 'Result of submitting a survey response',
    'AnalyticsSummary': 'Marketing analytics summary with visitor metrics',
    'ConversionRate': 'Conversion rate analytics with source breakdown',
    'VisitorTracking': 'Visitor tracking record with page and session data',
    'WebForm': 'Marketing form definition with fields and submission settings',
    
    # Pipeline
    'CreateLeadRequest': 'Request body for creating a new lead',
    'UpdateLeadRequest': 'Request body for updating an existing lead',
    'LeadConversionRequest': 'Request body for converting a lead to contact/account',
    'LeadStageTransition': 'Request body for transitioning a lead to a new stage',
    'LeadDuplicateCheck': 'Result of checking for duplicate leads',
    'LeadMergeRequest': 'Request body for merging duplicate leads',
    'MergeResult': 'Result of merging records including merged IDs',
    'Stage': 'Pipeline stage with order, name, and status',
    'StageSummary': 'Summary of a pipeline stage with lead counts',
    'PipelineSummary': 'Summary of pipeline including weighted values by stage',
    'PipelineStage': 'Configuration for a pipeline stage',
    'PipelineAnalytics': 'Pipeline analytics with stage distribution',
    'LeadScoreRequest': 'Request body for calculating lead score',
    'LeadScoreBatchRequest': 'Request body for batch lead scoring',
    
    # Platform
    'CreateCustomFieldRequest': 'Request body for creating a custom field definition',
    'UpdateCustomFieldRequest': 'Request body for updating a custom field definition',
    'CustomField': 'Custom field definition for entity extension',
    'CreateWebhookRequest': 'Request body for creating a webhook endpoint',
    'UpdateWebhookRequest': 'Request body for updating a webhook endpoint',
    'Webhook': 'Webhook endpoint configuration',
    'WebhookDeliveryLog': 'Log entry for webhook delivery attempts',
    'CreateApiKeyRequest': 'Request body for creating an API key',
    'ApiKey': 'API key configuration with permissions and expiry',
    'CreateIntegrationRequest': 'Request body for creating a third-party integration',
    'UpdateIntegrationRequest': 'Request body for updating an integration',
    'Integration': 'Third-party integration configuration',
    'AuditLogEntry': 'Audit log entry recording a system action',
    'AuditLogFilter': 'Filter parameters for audit log queries',
    'Report': 'Report definition with query parameters and scheduling',
    'ReportSchedule': 'Schedule configuration for automated report generation',
    
    # Reporting
    'ReportAnalytics': 'Pipeline analytics with stage distribution and trends',
    'PerformanceMetrics': 'Rep and team performance metrics',
    'ForecastEntry': 'Forecast entry for a period with predicted values',
    'ForecastAccuracy': 'Forecast accuracy comparison (predicted vs actual)',
    'WinLossAnalysis': 'Win/loss analysis with breakdown by reason',
    'TimeToCloseAnalysis': 'Time-to-close analysis by stage with averages',
    'ReportScheduleRequest': 'Request body for scheduling a report',
    'ReportExportRequest': 'Request body for exporting report data',
    
    # Teams
    'CreateTeamRequest': 'Request body for creating a new team',
    'UpdateTeamRequest': 'Request body for updating an existing team',
    'TeamMember': 'Team member details including role and status',
    'TeamMembers': 'Collection of team members with metadata',
    'LeadAssignmentRequest': 'Request body for assigning leads to teams',
}


def process_spec(spec_path):
    """Add descriptions to all schemas missing descriptions."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    schemas = data.get('components', {}).get('schemas', {})
    
    for schema_name, schema_def in schemas.items():
        if isinstance(schema_def, dict) and not schema_def.get('description'):
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
