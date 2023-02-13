#![allow(missing_docs)]
use async_graphql::SimpleObject;
use chrono::Utc;
use fake::{Dummy, Fake};
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::users::model as user_model;

/// The `RoleGrant` GraphQL and Database Model
#[derive(
    Clone,
    Debug,
    Dummy,
    Eq,
    PartialEq,
    DeriveEntityModel,
    Deserialize,
    Serialize,
    SimpleObject,
    PolarClass,
)]
#[sea_orm(table_name = "role_grants")]
pub struct Model {
    
}