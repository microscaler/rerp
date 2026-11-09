//! LifeModels for schema `messaging`: one file per table in `openapi/crm/docs/data-model/messaging.yaml`, each with
//! `#[schema_name = "messaging"]`. `python3 openapi/crm/tools/data_model.py --models` compares them with the design.

pub mod entity_registry {
    include!(concat!(env!("OUT_DIR"), "/entity_registry.rs"));
}
