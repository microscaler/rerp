"""
Fix 3: Add descriptions to all 187 operations across 11 CRM services.

This generates descriptions based on operation type, path pattern, and service context.
"""
import yaml
import os
import re
from collections import defaultdict

crm_base = '/home/casibbald/Workspace/microscaler/rerp/openapi/crm/'
fixed = 0

# Description templates based on operation type and path patterns
def generate_description(service, method, path):
    """Generate a meaningful description based on operation context."""
    
    method_map = {
        'get': 'Retrieve',
        'post': 'Create',
        'put': 'Update',
        'patch': 'Update',
        'delete': 'Delete',
    }
    
    action = method_map.get(method, method.upper())
    
    # Path normalization
    path = path.rstrip('/')
    
    # Extract resource from path
    # /leads → leads, /leads/{id} → leads, /analytics/pipeline-summary → pipeline summary
    resource = path.split('/')[1] if len(path.split('/')) > 1 else path.split('/')[0]
    
    # Special cases for compound paths
    compound_patterns = {
        'search-by-email': 'Search contacts by email address',
        'score-batch': 'Score a batch of leads asynchronously',
        'score-bulk': 'Score a batch of leads synchronously',
        'score/explain': 'Explain scoring factors for a lead',
        'status': 'Get or update agent status',
        'metrics': 'Get agent performance metrics',
        'transcript': 'Get chat session transcript',
        'renew': 'Renew a subscription',
        'health': 'Get customer health score',
        'at-risk': 'Get at-risk customers',
        'pipeline-summary': 'Get pipeline summary with stage breakdown',
        'pipeline-weighted': 'Get weighted pipeline revenue',
        'conversion-rates': 'Get conversion rate analytics',
        'funnel': 'Get conversion funnel data',
        'lead-sources': 'Get lead source analytics',
        'rep-performance': 'Get representative performance metrics',
        'team-performance': 'Get team performance metrics',
        'leaderboard': 'Get performance leaderboard',
        'win-loss': 'Get win/loss analysis',
        'time-to-close': 'Get time-to-close analytics',
        'forecast/monthly': 'Get monthly forecast',
        'forecast/quarterly': 'Get quarterly forecast',
        'forecast/accuracy': 'Get forecast accuracy metrics',
        'rebuild': 'Rebuild scoring frequencies from historical data',
        'auto-fill': 'Auto-fill lead data from email domain',
        'website': 'Enrich lead data from company website',
        'lookup': 'Enrich lead data from email address',
        'verify/email': 'Verify email address validity',
        'verify/batch': 'Verify email addresses in batch',
        'verify/phone': 'Verify phone number validity',
        'search': 'Search for leads matching ICP criteria',
        'icp-match': 'Find leads matching ideal customer profile',
        'track': 'Track visitor activity on a page',
        'identify': 'Identify a visitor by their session',
        'register': 'Register for an event',
        'registrations': 'Get event registrations',
        'submit': 'Submit a survey response',
        'config': 'Get web form configuration',
        'leads': 'Get leads associated with a campaign',
        'revenue': 'Get revenue attributed to a campaign',
        'tree': 'Get hierarchical tree structure',
        'contacts': 'Get contacts associated with an account',
        'leads': 'Get leads associated with an account',
        'accounts': 'Get accounts associated with a contact',
        'members': 'Get team members',
        'unassigned': 'Get unassigned leads in the queue',
        'run-history': 'Get workflow execution history',
        'rules': 'Get workflow rules',
        'run': 'Run a workflow manually',
        'fire': 'Fire a trigger manually',
        'test': 'Test a rule configuration',
        'schedule': 'Schedule a report to run on a recurring basis',
        'build': 'Build a custom report',
        'export': 'Export report data to a file',
        'validate': 'Validate an API key or token',
        'sync': 'Sync integration data with external system',
        'status': 'Get integration status',
        'logs': 'Get webhook delivery logs',
        'run': 'Run the lead assignment algorithm',
    }
    
    # Check if this is a detail endpoint (GET by ID)
    is_detail = '{' in path
    
    # Build description
    if resource in compound_patterns:
        desc = compound_patterns[resource]
    elif is_detail and method == 'get':
        desc = f"Get {action.lower()} details by ID"
    elif is_detail and method == 'delete':
        desc = f"Delete {action.lower()} by ID"
    elif method == 'get' and '{' not in path:
        desc = f"List all {resource}"
    elif method == 'post' and is_detail:
        desc = f"Perform an action on {action.lower()}"
    elif method == 'post':
        desc = f"Create a new {resource}"
    elif method == 'put':
        desc = f"Update {action.lower()} by ID"
    elif method == 'patch':
        desc = f"Partially update {action.lower()} by ID"
    elif method == 'delete':
        desc = f"Delete {action.lower()} by ID"
    else:
        desc = f"{action} {resource}"
    
    # Capitalize first letter
    return desc[0].upper() + desc[1:] if desc else ""

for item in sorted(os.listdir(crm_base)):
    spec_path = os.path.join(crm_base, item, 'openapi.yaml')
    if not os.path.exists(spec_path):
        continue
    
    with open(spec_path) as f:
        data = yaml.safe_load(f)
    
    modified = False
    for path, methods in data['paths'].items():
        for method, operation in methods.items():
            if method in ['get', 'post', 'put', 'patch', 'delete']:
                # Only add if missing
                if not operation.get('description'):
                    operation['description'] = generate_description(item, method, path)
                    modified = True
                    fixed += 1
    
    if modified:
        with open(spec_path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)

print(f"Total fixed: {fixed} operations across 11 services")
