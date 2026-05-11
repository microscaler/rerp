"""
Fix: Add response examples to all 63 missing 200/201 responses.

The schema $ref is nested: content.application-json.schema.$ref
So we must look at json_schema.get('schema', {}).get('$ref'), not json_schema.get('$ref').
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Example templates keyed by schema name
RESPONSE_EXAMPLES = {
    'Account': {'id': '00000000-0000-0000-0000-000000000001', 'name': 'Acme Corp', 'website': 'https://acme.com', 'industry': 'Technology', 'size': 'medium'},
    'Contact': {'id': '00000000-0000-0000-0000-000000000002', 'first_name': 'John', 'last_name': 'Doe', 'email': 'john@acme.com', 'phone': '+1-555-0123'},
    'Lead': {'id': '00000000-0000-0000-0000-000000000003', 'name': 'Jane Smith', 'email': 'jane@example.com', 'company': 'Example Inc', 'source': 'website'},
    'Stage': {'id': '00000000-0000-0000-0000-000000000004', 'name': 'Qualification', 'sequence': 1, 'probability': 20},
    'Team': {'id': '00000000-0000-0000-0000-000000000005', 'name': 'Sales Team Alpha', 'description': 'Enterprise sales team'},
    'Workflow': {'id': '00000000-0000-0000-0000-000000000006', 'name': 'Lead Assignment', 'description': 'Auto-assign leads based on region', 'enabled': True},
    'Rule': {'id': '00000000-0000-0000-0000-000000000007', 'name': 'Assign to region A', 'workflow_id': '00000000-0000-0000-0000-000000000006'},
    'Trigger': {'id': '00000000-0000-0000-0000-000000000008', 'name': 'On Lead Create', 'type': 'event', 'enabled': True},
    'ChatSession': {'id': '00000000-0000-0000-0000-000000000009', 'visitor_id': 'v123', 'agent_id': 'a001', 'status': 'active'},
    'ChatMessage': {'id': '00000000-0000-0000-0000-000000000010', 'session_id': '00000000-0000-0000-0000-000000000009', 'message': 'Hello!', 'sender_type': 'visitor'},
    'Agent': {'id': '00000000-0000-0000-0000-000000000011', 'name': 'Agent Smith', 'is_online': True, 'max_concurrent': 5},
    'Campaign': {'id': '00000000-0000-0000-0000-000000000012', 'name': 'Q1 Email Campaign', 'type': 'email', 'active': True},
    'Subscription': {'id': '00000000-0000-0000-0000-000000000013', 'customer_id': 'c001', 'plan': 'professional', 'status': 'active'},
    'Goal': {'id': '00000000-0000-0000-0000-000000000014', 'name': 'Close 10 deals', 'target_value': 10, 'period': 'quarterly'},
    'CustomField': {'id': '00000000-0000-0000-0000-000000000015', 'entity': 'lead', 'name': 'priority', 'type': 'string'},
    'Webhook': {'id': '00000000-0000-0000-0000-000000000016', 'url': 'https://example.com/hook', 'events': ['lead.created'], 'enabled': True},
    'ApiKey': {'id': '00000000-0000-0000-0000-000000000017', 'name': 'Production API', 'enabled': True, 'created_at': '2024-01-01T00:00:00Z'},
    'Integration': {'id': '00000000-0000-0000-0000-000000000018', 'name': 'Slack Integration', 'type': 'messaging', 'enabled': True},
    'Report': {'id': '00000000-0000-0000-0000-000000000019', 'name': 'Pipeline Report', 'type': 'pipeline', 'schedule': 'weekly'},
    'UTMCampaign': {'id': '00000000-0000-0000-0000-000000000020', 'name': 'Q1 Email Blast', 'active': True, 'sequence': 1},
    'UTMMedium': {'id': '00000000-0000-0000-0000-000000000021', 'name': 'email', 'active': True},
    'UTMSource': {'id': '00000000-0000-0000-0000-000000000022', 'name': 'newsletter', 'active': True},
    'Badge': {'id': '00000000-0000-0000-0000-000000000023', 'name': 'Top Performer', 'description': 'Awarded for exceeding targets', 'icon': 'trophy'},
    'UserBadge': {'id': '00000000-0000-0000-0000-000000000024', 'user_id': 'u001', 'badge_id': '00000000-0000-0000-0000-000000000023', 'earned_at': '2024-01-15T10:00:00Z'},
    'Summary': {'total': 150, 'active': 120, 'new': 30, 'closed': 100},
    'Analytics': {'total_visitors': 1000, 'page_views': 5000, 'unique_visitors': 750, 'bounce_rate': 35.5},
    'PipelineSummary': {'total_opportunities': 50, 'total_revenue': 250000, 'total_weighted_revenue': 125000, 'stages': []},
    'LeadScoreSummary': {'hot': 10, 'warm': 25, 'cold': 50, 'total_scored': 85},
    'LeadScoreRanked': {'id': '00000000-0000-0000-0000-000000000003', 'score': 95, 'rank': 1, 'level': 'hot'},
    'ScoreExplanation': {'score': 95, 'factors': [{'factor': 'engagement', 'weight': 40}, {'factor': 'demographics', 'weight': 55}]},
    'ScoreThresholds': {'hot': 80, 'warm': 50, 'cold': 20},
    'ConversionResult': {'success': True, 'contact_id': '00000000-0000-0000-0000-000000000002', 'account_id': '00000000-0000-0000-0000-000000000001'},
    'DuplicateDetectionResult': {'is_duplicate': False, 'matches': []},
    'MergeResult': {'success': True, 'kept_id': '00000000-0000-0000-0000-000000000001', 'merged_id': '00000000-0000-0000-0000-000000000002'},
    'WorkflowExecutionResult': {'workflow_id': '00000000-0000-0000-0000-000000000006', 'rules_triggered': 3, 'actions_executed': 2, 'status': 'completed'},
    'RuleTestResult': {'rule_id': '00000000-0000-0000-0000-000000000007', 'condition_met': True, 'actions_fired': 1},
    'ScheduledAction': {'id': '00000000-0000-0000-0000-000000000025', 'scheduled_at': '2024-02-01T09:00:00Z', 'status': 'pending'},
    'ActionUpcoming': {'id': '00000000-0000-0000-0000-000000000025', 'scheduled_at': '2024-02-01T09:00:00Z', 'repeat_interval': 'weekly'},
    'VerificationResult': {'email': 'test@example.com', 'valid': True, 'disposable': False, 'role_based': False},
    'EnrichmentResult': {'company': 'Example Inc', 'person': 'John Doe', 'confidence': 0.95},
    'EnrichmentLookupResult': {'company': 'Example Inc', 'industry': 'Technology', 'size': 'medium'},
    'MiningSearchResult': {'total_matches': 150, 'leads': []},
    'ICPMatchResult': {'total_matches': 50, 'leads': []},
    'AnalyticsSummary': {'visitors': 1000, 'page_views': 5000, 'unique_visitors': 750},
    'ConversionRate': {'overall_rate': 15.5, 'by_source': {'website': 18.2, 'email': 12.3}},
    'FunnelData': {'stages': [{'name': 'visit', 'count': 1000}, {'name': 'signup', 'count': 200}, {'name': 'purchase', 'count': 50}]},
    'LeadSourceReport': {'total_leads': 500, 'by_source': {'website': 200, 'email': 150, 'referral': 100, 'social': 50}},
    'RepPerformance': {'rep_id': 'r001', 'rep_name': 'Jane Doe', 'deals_closed': 10, 'revenue': 125000},
    'TeamPerformance': {'team_id': 't001', 'team_name': 'Sales Alpha', 'deals_closed': 45, 'revenue': 500000},
    'WinLossAnalysis': {'total_opportunities': 100, 'won': 60, 'lost': 40, 'by_reason': {'price': 20, 'features': 15, 'competitor': 5}},
    'TimeToCloseAnalysis': {'average_days': 30, 'by_stage': [{'stage': 'qualification', 'avg_days': 5}, {'stage': 'proposal', 'avg_days': 15}]},
    'ForecastEntry': {'period': '2024-Q2', 'predicted_revenue': 500000, 'confidence': 0.85},
    'ForecastAccuracyEntry': {'period': '2024-Q1', 'forecasted': 450000, 'actual': 500000, 'accuracy': 90},
    'LeaderboardEntry': {'rank': 1, 'rep_id': 'r001', 'rep_name': 'Jane Doe', 'deals_closed': 10, 'revenue': 125000},
    'AtRiskCustomer': {'customer_id': 'c001', 'chance_of_churn': 0.75, 'risk_factors': ['low_engagement', 'support_tickets']},
    'HealthScore': {'customer_id': 'c001', 'overall_score': 75, 'engagement_score': 80, 'support_score': 70},
    'RenewalSummary': {'total_upcoming': 50, 'completed': 30, 'revenue_impact': 150000},
    'MRRMetrics': {'current_mrr': 100000, 'new_mrr': 15000, 'expansion_mrr': 5000, 'churn_mrr': -2000},
    'ChurnMetrics': {'total_subscriptions': 500, 'churned_subscriptions': 25, 'churn_rate': 5.0},
    'BadgeResult': {'badge_id': '00000000-0000-0000-0000-000000000023', 'recipient_id': 'u001', 'earned_at': '2024-01-15T10:00:00Z'},
    'QueueItem': {'id': '00000000-0000-0000-0000-000000000003', 'name': 'John Doe', 'email': 'john@example.com', 'company': 'Acme Corp'},
    'AgentMetrics': {'agent_id': 'a001', 'avg_response_time': 30, 'satisfaction_score': 4.5, 'active_sessions': 3},
    'AgentStatus': {'agent_id': 'a001', 'is_online': True, 'active_sessions': 3, 'max_concurrent': 5},
    'AgentQueueItem': {'id': 'v123', 'name': 'Visitor', 'waiting_time': 120, 'referrer': 'website'},
    'AuditLogEntry': {'id': '00000000-0000-0000-0000-000000000026', 'action': 'update', 'entity': 'lead', 'entity_id': '00000000-0000-0000-0000-000000000003'},
    'WebhookDeliveryLog': {'id': '00000000-0000-0000-0000-000000000027', 'webhook_id': '00000000-0000-0000-0000-000000000016', 'status': 'success'},
    'CampaignLeadResult': {'lead_id': '00000000-0000-0000-0000-000000000003', 'campaign_id': '00000000-0000-0000-0000-000000000012', 'attribution': 'first_touch'},
    'EventRegistration': {'id': '00000000-0000-0000-0000-000000000028', 'event_id': 'e001', 'contact_id': 'c001', 'status': 'confirmed'},
    'FormSubmissionResult': {'form_id': 'f001', 'submission_id': 's001', 'status': 'success'},
    'SurveyResponse': {'id': '00000000-0000-0000-0000-000000000029', 'survey_id': 'survey001', 'respondent_email': 'user@example.com'},
    'TeamMember': {'id': '00000000-0000-0000-0000-000000000030', 'user_id': 'u001', 'team_id': 't001', 'role': 'member'},
    'TeamMembers': {'team_id': 't001', 'members': []},
    'VisitorTracking': {'id': '00000000-0000-0000-0000-000000000031', 'visitor_id': 'v001', 'url_visited': '/pricing', 'referrer': 'google.com'},
    'WebForm': {'id': 'f001', 'name': 'Contact Form', 'fields': ['name', 'email', 'message']},
    'WeightedPipeline': {'total_opportunities': 50, 'total_revenue': 250000, 'total_weighted_revenue': 125000},
    'LeadScore': {'id': '00000000-0000-0000-0000-000000000003', 'score': 95, 'level': 'hot'},
    'EnrichmentLookupResult': {'company': 'Target Corp', 'person': 'John Doe', 'confidence': 0.92},
    'EmailVerificationResult': {'email': 'john@target.com', 'valid': True, 'disposable': False},
    'PhoneVerificationResult': {'phone': '+1-555-0123', 'valid': True, 'carrier': 'Verizon', 'phone_type': 'mobile'},
    'ChatTranscript': {'session_id': '00000000-0000-0000-0000-000000000009', 'messages': [], 'duration_seconds': 300},
    'ScheduleActionRequest': None,
    'RecurringActionRequest': None,
    'AccountContactSummary': {'account_id': '00000000-0000-0000-0000-000000000001', 'contacts': []},
    'AccountReport': {'account_id': '00000000-0000-0000-0000-000000000001', 'total_deals': 10, 'total_revenue': 50000},
    'WorkflowExecution': {'id': '00000000-0000-0000-0000-000000000032', 'workflow_id': '00000000-0000-0000-0000-000000000006', 'triggered_by': 'manual', 'started_at': '2024-01-01T10:00:00Z'},
    'WorkflowSummary': {'id': '00000000-0000-0000-0000-000000000006', 'name': 'Lead Assignment', 'enabled': True, 'total_executions': 150},
    'ExecutionLog': {'id': '00000000-0000-0000-0000-000000000033', 'workflow_id': '00000000-0000-0000-0000-000000000006', 'status': 'completed', 'started_at': '2024-01-01T10:00:00Z'},
    'ContactSummary': {'id': '00000000-0000-0000-0000-000000000002', 'total_deals': 5, 'total_revenue': 25000, 'last_activity': '2024-01-15T10:00:00Z'},
    'ContactDuplicateCheck': {'is_duplicate': False, 'potential_matches': []},
    'CustomerHealthScore': {'customer_id': 'c001', 'score': 75, 'engagement_score': 80, 'support_score': 70, 'usage_score': 75},
    'ScoringThresholds': {'hot': 80, 'warm': 50, 'cold': 20},
    'EnrichmentResponse': {'company': {'name': 'Example Inc', 'domain': 'example.com', 'industry': 'Technology'}, 'person': {'name': 'John Doe', 'title': 'CTO'}},
    'DiscoveredLead': {'name': 'John Doe', 'email': 'john@target.com', 'company': 'Target Corp', 'title': 'CTO'},
    'SavedReport': {'id': '00000000-0000-0000-0000-000000000034', 'name': 'Monthly Pipeline', 'type': 'pipeline'},
    'ReportSchedule': {'id': '00000000-0000-0000-0000-000000000035', 'report_id': '00000000-0000-0000-0000-000000000019', 'schedule': 'weekly', 'enabled': True},
    'ConversionResult': {'success': True, 'contact_id': '00000000-0000-0000-0000-000000000002', 'account_id': '00000000-0000-0000-0000-000000000001'},
    'AssignmentResult': {'assigned_count': 5, 'failed_count': 0, 'details': []},
    'UnassignedLead': {'lead_id': '00000000-0000-0000-0000-000000000003', 'name': 'John Doe', 'source': 'website'},
    'PaginatedAccount': 'Account', 'PaginatedContact': 'Contact', 'PaginatedLead': 'Lead', 'PaginatedStage': 'Stage', 'PaginatedTeam': 'Team',
    'PaginatedWorkflow': 'Workflow', 'PaginatedRule': 'Rule', 'PaginatedTrigger': 'Trigger', 'PaginatedChatSession': 'ChatSession',
    'PaginatedChatMessage': 'ChatMessage', 'PaginatedAgent': 'Agent', 'PaginatedCampaign': 'Campaign', 'PaginatedSubscription': 'Subscription',
    'PaginatedGoal': 'Goal', 'PaginatedCustomField': 'CustomField', 'PaginatedWebhook': 'Webhook', 'PaginatedApiKey': 'ApiKey',
    'PaginatedIntegration': 'Integration', 'PaginatedReport': 'Report', 'PaginatedUTMCampaign': 'UTMCampaign', 'PaginatedUTMMedium': 'UTMMedium',
    'PaginatedUTMSource': 'UTMSource', 'PaginatedEventRegistration': 'EventRegistration', 'PaginatedVisitorTracking': 'VisitorTracking',
    'PaginatedCampaignLeadResult': 'CampaignLeadResult', 'PaginatedPipelineSummary': 'PipelineSummary', 'PaginatedScoringFrequency': None,
    'PaginatedLeadScoreSummary': 'LeadScoreSummary', 'PaginatedLeadScoreRanked': 'LeadScoreRanked', 'PaginatedConversionrate': 'ConversionRate',
    'PaginatedForecastentry': 'ForecastEntry', 'PaginatedLeadSourceReport': 'LeadSourceReport', 'PaginatedLeaderboardEntry': 'LeaderboardEntry',
    'PaginatedForecastAccuracyEntry': 'ForecastAccuracyEntry', 'PaginatedAgentStatus': 'AgentStatus', 'PaginatedAgentQueueItem': 'AgentQueueItem',
    'PaginatedPipelinesummary': 'PipelineSummary', 'PaginatedWebhookDeliveryLog': 'WebhookDeliveryLog', 'PaginatedAuditLogEntry': 'AuditLogEntry',
    'PaginatedUnassignedLeads': 'UnassignedLead', 'PaginatedScheduledactions': 'ScheduledAction', 'PaginatedUserbadges': 'UserBadge',
    'PaginatedAccountcontactsummarys': 'AccountContactSummary', 'PaginatedResponse': None,
}


def get_paginated_template(base_type):
    """Get a paginated response example for a given base type."""
    item_template = RESPONSE_EXAMPLES.get(base_type)
    if item_template is None:
        return {'items': [], 'total': 0, 'page': 1, 'per_page': 20}
    if isinstance(item_template, str):
        example = RESPONSE_EXAMPLES.get(item_template)
        if example:
            return {'items': [example], 'total': 1, 'page': 1, 'per_page': 20}
    elif item_template:
        return {'items': [item_template], 'total': 1, 'page': 1, 'per_page': 20}
    return {'items': [], 'total': 0, 'page': 1, 'per_page': 20}


def process_spec(spec_path):
    """Add response examples to all 200/201 responses missing them."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    for path, methods in data['paths'].items():
        for method, op in methods.items():
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            
            responses = op.get('responses', {})
            for sc in ['200', '201']:
                if sc not in responses:
                    continue
                content = responses[sc].get('content', {})
                json_schema = content.get('application/json', {})
                
                # Skip if already has examples
                if json_schema.get('examples'):
                    continue
                
                # The $ref is nested inside a 'schema' object
                schema_obj = json_schema.get('schema', {})
                ref = schema_obj.get('$ref', '')
                if not ref:
                    continue
                
                schema_name = ref.split('/')[-1]
                
                # Handle Paginated* types
                if schema_name.startswith('Paginated'):
                    base_name = schema_name.replace('Paginated', '')
                    template = get_paginated_template(base_name)
                    if template:
                        json_schema['examples'] = {'default': template}
                        modified += 1
                        continue
                
                # Handle known entity types
                if schema_name in RESPONSE_EXAMPLES:
                    example = RESPONSE_EXAMPLES[schema_name]
                    if example is not None:
                        json_schema['examples'] = {'default': example}
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
        print(f"  {item}: {fixed} response examples added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} response examples added")
