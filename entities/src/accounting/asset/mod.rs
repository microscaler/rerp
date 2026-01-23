//! Asset Management Service Entities
//!
//! This module contains entities for asset management including:
//! - Fixed assets
//! - Asset depreciation
//! - Asset categories
//! - Asset transactions

pub mod asset;
pub mod asset_category;
pub mod asset_depreciation;
pub mod asset_transaction;

pub use asset::Asset;
pub use asset_category::AssetCategory;
pub use asset_depreciation::AssetDepreciation;
pub use asset_transaction::AssetTransaction;
