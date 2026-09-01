// Impl controller registration for the CRM pipeline service (ADR 0001 Layer 2:
// production registers only user-owned implementations; generated example
// handlers are forbidden). Routes without an entry here return 404 until they
// have a real implementation.

use brrtrouter::dispatcher::Dispatcher;
use brrtrouter::spec::RouteMeta;
use brrtrouter::typed::spawn_typed_with_stack_size_and_name;

/// Register impl controllers (overrides gen stubs per ADR 0001 Layer 2).
///
/// # Safety
/// Spawns handler coroutines. Callers must ensure the coroutine runtime is initialized.
pub unsafe fn register_impl(dispatcher: &mut Dispatcher, routes: &[RouteMeta]) {
    for route in routes {
        match route.handler_name.as_ref() {
            "list_leads" => {
                let tx = spawn_typed_with_stack_size_and_name(
                    crate::controllers::list_leads::ListLeadsController,
                    0x8000,
                    Some(route.handler_name.as_ref()),
                );
                dispatcher.add_route(route.clone(), tx);
            }
            "get_lead" => {
                let tx = spawn_typed_with_stack_size_and_name(
                    crate::controllers::get_lead::GetLeadController,
                    0x8000,
                    Some(route.handler_name.as_ref()),
                );
                dispatcher.add_route(route.clone(), tx);
            }
            "update_lead" => {
                let tx = spawn_typed_with_stack_size_and_name(
                    crate::controllers::update_lead::UpdateLeadController,
                    0x8000,
                    Some(route.handler_name.as_ref()),
                );
                dispatcher.add_route(route.clone(), tx);
            }
            "change_stage" => {
                let tx = spawn_typed_with_stack_size_and_name(
                    crate::controllers::change_stage::ChangeStageController,
                    0x8000,
                    Some(route.handler_name.as_ref()),
                );
                dispatcher.add_route(route.clone(), tx);
            }
            "list_stages" => {
                let tx = spawn_typed_with_stack_size_and_name(
                    crate::controllers::list_stages::ListStagesController,
                    0x8000,
                    Some(route.handler_name.as_ref()),
                );
                dispatcher.add_route(route.clone(), tx);
            }
            "pipeline_summary" => {
                let tx = spawn_typed_with_stack_size_and_name(
                    crate::controllers::pipeline_summary::PipelineSummaryController,
                    0x8000,
                    Some(route.handler_name.as_ref()),
                );
                dispatcher.add_route(route.clone(), tx);
            }
            _ => {}
        }
    }
}
