"""
Fix: Add descriptions to ALL remaining schemas across 11 CRM specs.

Covers:
- Request types (CreateXxx, UpdateXxx, XxxRequest)
- Entity types (WorkflowExecution, ContactHierarchy, etc.)
- Enum types (TriggerType, ActionType, Status, etc.)
- Paginated types (allOf inheritance)
- Collection types (ScheduledActions, etc.)
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Full descriptions for ALL remaining types
SCHEMA_DESCRIPTIONS = {
    # === AUTOMATION ===
    'CreateWorkflowRequest': 'Request body for creating a new workflow automation',
    'UpdateWorkflowRequest': 'Request body for updating an existing workflow',
    'CreateRuleRequest': 'Request body for creating a new workflow rule',
    'UpdateRuleRequest': 'Request body for updating an existing rule',
    'CreateTriggerRequest': 'Request body for creating a new workflow trigger',
    'UpdateTriggerRequest': 'Request body for updating an existing trigger',
    'WorkflowExecution': 'Record of a workflow execution event with timing and status',
    'ScheduleActionRequest': 'Request body for scheduling a one-time action',
    'RecurringActionRequest': 'Request body for scheduling a recurring action',
    'TriggerType': 'Enum of trigger types (event-based, schedule-based, webhook-based)',
    'ActionType': 'Enum of action types (notification, assign, update, create, etc.)',
    'ConditionOperator': 'Enum of condition operators (equals, contains, greater_than, etc.)',
    'ConditionValueType': 'Enum of condition value types (string, number, date, boolean)',
    'ExecutionStatus': 'Enum of workflow execution statuses (running, completed, failed)',
    'Workflowexecutions': 'Collection of workflow execution records',
    'Scheduledactions': 'Collection of scheduled action records',
    'PaginatedWorkflow': 'Paginated list of Workflow records',
    'PaginatedRule': 'Paginated list of Rule records',
    'PaginatedTrigger': 'Paginated list of Trigger records',
    
    # === CONTACTS ===
    'ContactHierarchy': 'Contact tree showing parent-child relationships',
    'MergeContactsRequest': 'Request body for merging two duplicate contacts',
    'PaginatedContact': 'Paginated list of Contact records',
    
    # === ENGAGEMENT ===
    'MRRMetrics': 'Monthly Recurring Revenue metrics with breakdown by source',
    'ChurnMetrics': 'Churn rate metrics including counts and revenue impact',
    'CreateGoalRequest': 'Request body for creating a new performance goal',
    'UpdateGoalRequest': 'Request body for updating an existing goal',
    'Badge': 'Badge definition with name, criteria, and icon',
    'UserBadge': 'Record of a user earning a badge with timestamp',
    'CustomerHealthScore': 'Customer health score with contributing sub-scores',
    'SubscriptionStatus': 'Enum of subscription statuses (active, cancelled, past_due)',
    'BillingCycle': 'Enum of billing cycles (monthly, quarterly, annual)',
    'GoalType': 'Enum of goal types (revenue, activity, retention)',
    'RiskLevel': 'Enum of risk levels (low, medium, high, critical)',
    'Userbadges': 'Collection of user badge records',
    'PaginatedSubscription': 'Paginated list of Subscription records',
    'PaginatedGoal': 'Paginated list of Goal records',
    'PaginatedBadge': 'Paginated list of Badge records',
    
    # === INTELLIGENCE ===
    'ScoringThresholds': 'Score thresholds defining hot/warm/cold levels',
    'UpdateThresholdsRequest': 'Request body for updating lead scoring thresholds',
    'EnrichmentLookupRequest': 'Request body for enriching lead from email lookup',
    'WebsiteEnrichmentRequest': 'Request body for enriching lead from website lookup',
    'BatchEnrichmentRequest': 'Request body for batch-enriching leads from external sources',
    'EmailVerificationResult': 'Result of email verification including deliverability check',
    'PhoneVerificationResult': 'Result of phone verification including carrier detection',
    'LeadMiningRequest': 'Request body for searching ICP-matched leads',
    'ICPMatchRequest': 'Request body for matching leads against ideal customer profile',
    'ScoreBucket': 'Enum of score buckets (hot, warm, cold, unscored)',
    'CompanySize': 'Enum of company size categories (micro, small, medium, enterprise)',
    'PersonRole': 'Enum of person roles (decision_maker, influencer, end_user)',
    'PersonSeniority': 'Enum of seniority levels (junior, mid, senior, executive, c_suite)',
    'EnrichmentStatus': 'Enum of enrichment statuses (pending, completed, failed)',
    'EnrichmentSource': 'Enum of data enrichment sources (clearbit, hunter, linkedin)',
    'PhoneType': 'Enum of phone types (mobile, landline, toll_free)',
    'PaginatedScoringFrequency': 'Paginated list of ScoringFrequency records',
    'PaginatedLeadScoreSummary': 'Paginated list of LeadScoreSummary records',
    'PaginatedLeadScoreRanked': 'Paginated list of LeadScoreRanked records',
    
    # === LIVECHAT ===
    'CreateChatRequest': 'Request body for creating a new chat session',
    'SendChatRequest': 'Request body for sending a chat message',
    'ConvertChatRequest': 'Request body for converting a chat session to a lead',
    'ChatTranscript': 'Chat transcript with message history and session metadata',
    'SendMessageRequest': 'Request body for sending a message in a chat session',
    'CreateAgentRequest': 'Request body for creating a new agent',
    'UpdateAgentRequest': 'Request body for updating an existing agent',
    'AgentStatusRequest': 'Request body for updating agent online status',
    'ChatStatus': 'Enum of chat statuses (waiting, active, closed, missed)',
    'SenderType': 'Enum of message sender types (agent, visitor, system)',
    'PaginatedAgent': 'Paginated list of Agent records',
    'PaginatedChatSession': 'Paginated list of ChatSession records',
    'PaginatedChatMessage': 'Paginated list of ChatMessage records',
    'PaginatedAgentStatus': 'Paginated list of AgentStatus records',
    'PaginatedAgentQueueItem': 'Paginated list of AgentQueueItem records',
    
    # === MARKETING ===
    'CreateUTMCampaignRequest': 'Request body for creating a new UTM campaign',
    'UpdateUTMCampaignRequest': 'Request body for updating an existing UTM campaign',
    'CreateUTMMediumRequest': 'Request body for creating a new UTM medium',
    'CreateUTMSourceRequest': 'Request body for creating a new UTM source',
    'FormSubmissionRequest': 'Request body for submitting a marketing form',
    'TrackPageViewRequest': 'Request body for tracking a visitor page view',
    'IdentifyVisitorRequest': 'Request body for identifying a returning visitor',
    'EventRegistrationRequest': 'Request body for registering for an event',
    'SurveyResponseRequest': 'Request body for submitting a survey response',
    'PaginatedEventRegistration': 'Paginated list of EventRegistration records',
    'PaginatedUTMCampaign': 'Paginated list of UTMCampaign records',
    'PaginatedUTMMedium': 'Paginated list of UTMMedium records',
    'PaginatedUTMSource': 'Paginated list of UTMSource records',
    'PaginatedVisitorTracking': 'Paginated list of VisitorTracking records',
    'PaginatedCampaignLeadResult': 'Paginated list of CampaignLeadResult records',
    
    # === PIPELINE ===
    'CreateStageRequest': 'Request body for creating a new pipeline stage',
    'UpdateStageRequest': 'Request body for updating an existing stage',
    'StageChangeRequest': 'Request body for transitioning a lead to a new stage',
    'ConvertLeadRequest': 'Request body for converting a lead to contact/account',
    'DuplicateDetectionRequest': 'Request body for checking lead duplicates by email/phone/name',
    'MergeLeadsRequest': 'Request body for merging duplicate leads',
    'PaginatedLead': 'Paginated list of Lead records',
    'PaginatedStage': 'Paginated list of Stage records',
    'PaginatedPipelineSummary': 'Paginated list of PipelineSummary records',
    
    # === PLATFORM ===
    'FieldType': 'Enum of custom field types (text, number, date, boolean, select, email, phone)',
    'AuditAction': 'Enum of audit action types (create, update, delete, login, logout)',
    'WebhookEvent': 'Enum of webhook event types (lead.created, contact.updated, etc.)',
    'WebhookDeliveryStatus': 'Enum of webhook delivery statuses (pending, sent, failed, retrying)',
    'IntegrationType': 'Enum of integration types (email, calendar, crm, accounting)',
    'SyncDirection': 'Enum of sync directions (inbound, outbound, bidirectional)',
    'SyncStatus': 'Enum of sync statuses (syncing, completed, failed, paused)',
    'PaginatedWebhook': 'Paginated list of Webhook records',
    'PaginatedApiKey': 'Paginated list of ApiKey records',
    'PaginatedAuditLogEntry': 'Paginated list of AuditLogEntry records',
    'PaginatedCustomField': 'Paginated list of CustomField records',
    'PaginatedIntegration': 'Paginated list of Integration records',
    'PaginatedWebhookDeliveryLog': 'Paginated list of WebhookDeliveryLog records',
    
    # === REPORTING ===
    'ReportBuildRequest': 'Request body for building a report with filters and groupings',
    'SavedReport': 'Saved report with configuration and cached results',
    'PaginatedPipelinesummary': 'Paginated list of PipelineSummary records',
    'PaginatedConversionrate': 'Paginated list of ConversionRate records',
    'PaginatedForecastentry': 'Paginated list of ForecastEntry records',
    'PaginatedLeadSourceReport': 'Paginated list of LeadSourceReport records',
    'PaginatedLeaderboardEntry': 'Paginated list of LeaderboardEntry records',
    'PaginatedForecastAccuracyEntry': 'Paginated list of ForecastAccuracyEntry records',
    
    # === TEAMS ===
    'AddTeamMemberRequest': 'Request body for adding a member to a team',
    'AssignmentResult': 'Result of lead assignment with counts and details',
    'PaginatedTeam': 'Paginated list of Team records',
    'UnassignedLead': 'Lead data for unassigned leads in the queue',
    'PaginatedUnassignedLeads': 'Paginated list of UnassignedLead records',
    
    # === ACCOUNTS ===
    'Accountcontactsummarys': 'Collection of account-contact summary records',
    'PaginatedAccount': 'Paginated list of Account records',
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
    if item.startswith('.') or item in ('CRM_ANALYSIS', 'docs'):
        continue
    if not os.path.isdir(os.path.join(crm_base, item)):
        continue
    
    spec_path = os.path.join(crm_base, item, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    fixed = process_spec(spec_path)
    if fixed:
        print(f"  {item}: {fixed} schema descriptions added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} schema descriptions added")
