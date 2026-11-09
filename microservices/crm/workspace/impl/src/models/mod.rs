//! LifeModels for schema `workspace`: one file per table in `openapi/crm/docs/data-model/workspace.yaml`, each with
//! `#[schema_name = "workspace"]`. `python3 openapi/crm/tools/data_model.py --models` compares them with the design.

pub mod entity_registry {
    include!(concat!(env!("OUT_DIR"), "/entity_registry.rs"));
}
