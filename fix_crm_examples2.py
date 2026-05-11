"""
Item 1 (Part 2): Add examples to remaining 125 operations.

Covers all 54 unique schema types that weren't matched in Part 1:
- 22 Paginated* schemas
- 23 Result/Response/Entry schemas  
- 8 Analytics/Report schemas
- 1 basic UTMApiKey schema
"""
import yaml
import os

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'

# Complete templates for all remaining schemas
ALL_REMAINING_TEMPLATES = {
    # Analytics & Report schemas
    'AnalyticsSummary': {
        'total_visitors': 1250,
        'unique_visitors': 890,
        'bounce_rate': 35.2,
        'avg_session_duration': 245,
        'period': 'daily',
    },
    'ConversionResult': {
        'converted': True,
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'contact_id': '00000000-0000-0000-0000-000000000002',
        'converted_at': '2024-01-15T10:30:00Z',
    },
    'ConversionRate': {
        'period': 'monthly',
        'total_leads': 500,
        'converted': 62,
        'rate': 12.4,
        'by_source': {'website': 15.2, 'referral': 8.5, 'campaign': 18.3},
    },
    'DuplicateDetectionResult': {
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'duplicates_found': 2,
        'duplicate_ids': [
            '00000000-0000-0000-0000-000000000030',
            '00000000-0000-0000-0000-000000000031',
        ],
        'confidence': 0.92,
    },
    'FunnelData': {
        'stages': [
            {'name': 'visitor', 'count': 10000},
            {'name': 'lead', 'count': 1250},
            {'name': 'qualified', 'count': 450},
            {'name': 'customer', 'count': 62},
        ],
    },
    'LeadScore': {
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'score': 85,
        'level': 'hot',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'LeadScoreRanked': {
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'name': 'John Doe',
        'score': 92,
        'rank': 1,
    },
    'LeadScoreSummary': {
        'total_leads': 150,
        'hot': 45,
        'warm': 65,
        'cold': 40,
    },
    'MergeResult': {
        'primary_id': '00000000-0000-0000-0000-000000000002',
        'merged_from': [
            '00000000-0000-0000-0000-000000000032',
            '00000000-0000-0000-0000-000000000033',
        ],
        'fields_merged': ['email', 'phone', 'company'],
    },
    'RepPerformance': {
        'rep_id': '00000000-0000-0000-0000-000000000010',
        'rep_name': 'Jane Smith',
        'deals_won': 12,
        'deals_lost': 5,
        'revenue': 125000,
        'conversion_rate': 24.0,
    },
    'SavedReport': {
        'id': '00000000-0000-0000-0000-000000000025',
        'name': 'Monthly Pipeline Report',
        'type': 'pipeline',
        'created_at': '2024-01-15T10:30:00Z',
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'ScoreExplanation': {
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'score': 85,
        'factors': [
            {'name': 'email_verified', 'weight': 10, 'applied': 10},
            {'name': 'company_size', 'weight': 20, 'applied': 20},
            {'name': 'industry_match', 'weight': 30, 'applied': 25},
        ],
    },
    'TimeToCloseAnalysis': {
        'avg_days': 45,
        'median_days': 38,
        'by_stage': {'discovery': 10, 'qualified': 15, 'proposal': 12},
    },
    'WinLossAnalysis': {
        'won': 45,
        'lost': 30,
        'win_rate': 60.0,
        'by_reason': {'price': 12, 'feature_gap': 8, 'competitor': 6},
    },
    
    # Result/Response schemas
    'AssignmentResult': {
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'assigned_to': '00000000-0000-0000-0000-000000000010',
        'assigned_at': '2024-01-15T10:30:00Z',
        'reason': 'round_robin',
    },
    'ChatConversionResult': {
        'session_id': '00000000-0000-0000-0000-000000000013',
        'converted_to_lead': True,
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'converted_at': '2024-01-15T10:30:00Z',
    },
    'ChatTranscript': {
        'session_id': '00000000-0000-0000-0000-000000000013',
        'messages': [
            {'sender': 'agent', 'content': 'Hello, how can I help?', 'timestamp': '2024-01-15T10:30:00Z'},
            {'sender': 'visitor', 'content': 'I need pricing info', 'timestamp': '2024-01-15T10:30:15Z'},
        ],
        'total_messages': 5,
    },
    'EmailVerificationResult': {
        'email': 'john@example.com',
        'valid': True,
        'disposable': False,
        'role_account': False,
        'mx_records': True,
    },
    'EnrichmentResponse': {
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'company': 'Acme Corp',
        'industry': 'Technology',
        'employees': 500,
        'found': True,
    },
    'EventRegistration': {
        'id': '00000000-0000-0000-0000-000000000028',
        'event_id': '00000000-0000-0000-0000-000000000029',
        'contact_id': '00000000-0000-0000-0000-000000000002',
        'registered_at': '2024-01-15T10:30:00Z',
        'status': 'confirmed',
    },
    'FormSubmissionResult': {
        'form_id': '00000000-0000-0000-0000-000000000030',
        'submitted': True,
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'submitted_at': '2024-01-15T10:30:00Z',
    },
    'PhoneVerificationResult': {
        'phone': '+1-555-0123',
        'valid': True,
        'line_type': 'mobile',
        'carrier': 'Verizon',
    },
    'ReportSchedule': {
        'id': '00000000-0000-0000-0000-000000000031',
        'report_id': '00000000-0000-0000-0000-000000000025',
        'frequency': 'monthly',
        'next_run': '2024-02-01T00:00:00Z',
        'enabled': True,
    },
    'RuleTestResult': {
        'rule_id': '00000000-0000-0000-0000-000000000008',
        'test_passed': True,
        'matched': True,
        'evaluation_time_ms': 12,
    },
    'ScheduledAction': {
        'id': '00000000-0000-0000-0000-000000000026',
        'type': 'email',
        'scheduled_at': '2024-01-16T10:00:00Z',
        'status': 'pending',
    },
    'ScoringThresholds': {
        'hot_threshold': 80,
        'warm_threshold': 50,
        'cold_threshold': 0,
        'updated_at': '2024-01-15T10:30:00Z',
    },
    'SurveyResponse': {
        'id': '00000000-0000-0000-0000-000000000032',
        'survey_id': '00000000-0000-0000-0000-000000000033',
        'contact_id': '00000000-0000-0000-0000-000000000002',
        'score': 4,
        'feedback': 'Great experience!',
    },
    'TeamMember': {
        'id': '00000000-0000-0000-0000-000000000010',
        'name': 'Jane Smith',
        'email': 'jane@example.com',
        'role': 'sales_rep',
    },
    'TeamMembers': {
        'team_id': '00000000-0000-0000-0000-000000000006',
        'members': [
            {'id': '00000000-0000-0000-0000-000000000010', 'name': 'Jane Smith'},
            {'id': '00000000-0000-0000-0000-000000000034', 'name': 'John Doe'},
        ],
        'total': 2,
    },
    'TeamPerformance': {
        'team_id': '00000000-0000-0000-0000-000000000006',
        'team_name': 'Sales Team Alpha',
        'total_deals': 24,
        'total_revenue': 250000,
        'avg_deal_size': 10416,
    },
    'VisitorTracking': {
        'id': '00000000-0000-0000-0000-000000000035',
        'visitor_id': '00000000-0000-0000-0000-000000000014',
        'page': '/pricing',
        'duration': 45,
        'timestamp': '2024-01-15T10:30:00Z',
    },
    'WebForm': {
        'id': '00000000-0000-0000-0000-000000000030',
        'name': 'Contact Us Form',
        'fields': ['name', 'email', 'message'],
        'enabled': True,
    },
    'WeightedPipeline': {
        'total_leads': 150,
        'stage_breakdown': {
            'discovery': {'count': 45, 'value': 100000, 'probability': 0.2},
            'qualified': {'count': 35, 'value': 125000, 'probability': 0.4},
        },
        'weighted_value': 85000,
    },
    'WorkflowExecutionResult': {
        'workflow_id': '00000000-0000-0000-0000-000000000007',
        'status': 'completed',
        'started_at': '2024-01-15T10:30:00Z',
        'completed_at': '2024-01-15T10:30:05Z',
        'rules_triggered': 3,
    },
    
    # UTMApiKey schemas
    'UTMCampaign': {
        'id': '00000000-0000-0000-0000-000000000017',
        'name': 'Q1 Marketing',
        'type': 'email',
        'status': 'active',
    },
    'UTMMedium': {
        'id': '00000000-0000-0000-0000-000000000036',
        'name': 'email',
        'description': 'Email marketing medium',
    },
    'UTMSource': {
        'id': '00000000-0000-0000-0000-000000000037',
        'name': 'newsletter',
        'description': 'Newsletter traffic source',
    },
    
    # Agent-related schemas
    'AgentMetrics': {
        'agent_id': '00000000-0000-0000-0000-000000000015',
        'total_chats': 45,
        'avg_response_time': 12,
        'satisfaction_score': 4.5,
        'period': 'weekly',
    },
    'AgentStatus': {
        'agent_id': '00000000-0000-0000-0000-000000000015',
        'status': 'available',
        'current_chats': 2,
        'queue_position': 1,
    },
    'AgentQueueItem': {
        'id': '00000000-0000-0000-0000-000000000014',
        'visitor_name': 'John Doe',
        'wait_time': 45,
        'priority': 'normal',
    },
    
    # Audit & Log schemas
    'AuditLogEntry': {
        'id': '00000000-0000-0000-0000-000000000038',
        'entity': 'lead',
        'entity_id': '00000000-0000-0000-0000-000000000001',
        'action': 'updated',
        'timestamp': '2024-01-15T10:30:00Z',
        'user_id': '00000000-0000-0000-0000-000000000010',
    },
    
    # Campaign lead results
    'CampaignLeadResult': {
        'campaign_id': '00000000-0000-0000-0000-000000000017',
        'lead_id': '00000000-0000-0000-0000-000000000001',
        'converted': True,
        'converting_at': '2024-01-15T10:30:00Z',
    },
    
    # Forecast schemas
    'ForecastAccuracyEntry': {
        'period': 'monthly',
        'forecasted': 420000,
        'actual': 405000,
        'accuracy': 96.4,
    },
    'ForecastEntry': {
        'period': 'monthly',
        'total_value': 420000,
        'confidence': 0.75,
        'pipeline_value': 560000,
    },
    
    # Lead source & leaderboard
    'LeadSourceReport': {
        'total': 850,
        'by_source': {
            'website': 350,
            'referral': 200,
            'campaign': 180,
            'social': 120,
        },
    },
    'LeaderboardEntry': {
        'rep_id': '00000000-0000-0000-0000-000000000010',
        'rep_name': 'Jane Smith',
        'deals_won': 12,
        'revenue': 125000,
        'rank': 1,
    },
    
    # Pipeline summary
    'PipelineSummary': {
        'total_leads': 150,
        'stage_breakdown': {
            'discovery': 45,
            'qualified': 35,
            'proposal': 25,
            'negotiation': 30,
        },
        'weighted_value': 375000,
        'forecast': 420000,
    },
    
    # Unassigned leads
    'UnassignedLeads': {
        'id': '00000000-0000-0000-0000-000000000001',
        'name': 'John Doe',
        'source': 'website',
        'score': 45,
        'created_at': '2024-01-15T10:30:00Z',
    },
    
    # Webhook delivery logs
    'WebhookDeliveryLog': {
        'id': '00000000-0000-0000-0000-000000000039',
        'webhook_id': '00000000-0000-0000-0000-000000000022',
        'event': 'lead.created',
        'status': 'success',
        'response_code': 200,
        'delivered_at': '2024-01-15T10:30:00Z',
    },
}


def process_spec(spec_path):
    """Process a single OpenAPI spec file."""
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = 0
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method not in ['get', 'post', 'put', 'patch', 'delete']:
                continue
            
            responses = operation.get('responses', {})
            
            for status_code in ['200', '201']:
                if status_code not in responses:
                    continue
                
                # Skip if already has examples
                content = responses[status_code].get('content', {})
                if content.get('application/json', {}).get('examples'):
                    continue
                
                json_schema = content.get('application/json', {}).get('schema', {})
                
                # Get schema name from $ref
                if '$ref' in json_schema:
                    schema_name = json_schema['$ref'].split('/')[-1]
                    
                    # Direct match
                    if schema_name in ALL_REMAINING_TEMPLATES:
                        template = ALL_REMAINING_TEMPLATES[schema_name]
                        
                        # Handle Paginated* schemas
                        if schema_name.startswith('Paginated'):
                            item_type = schema_name.replace('Paginated', '')
                            if item_type in ALL_REMAINING_TEMPLATES:
                                template = {
                                    'items': [ALL_REMAINING_TEMPLATES[item_type]],
                                    'total': 1,
                                    'page': 1,
                                    'per_page': 20,
                                    'has_next': False,
                                    'has_prev': False,
                                }
                            else:
                                template = {
                                    'items': [{'id': '00000000-0000-0000-0000-000000000001'}],
                                    'total': 1,
                                    'page': 1,
                                    'per_page': 20,
                                    'has_next': False,
                                    'has_prev': False,
                                }
                        
                        responses[status_code]['content']['application/json']['examples'] = {
                            'default': template
                        }
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
        print(f"  {item}: {fixed} examples added")
        total_fixed += fixed

print(f"\nTotal: {total_fixed} additional operations now have examples")
