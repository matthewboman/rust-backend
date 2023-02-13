#![allow(missing_docs)]

use async_graphql::SimpleObject;
use chrono::Utc;
use fake::{Dummy, Fake};
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// The User GraphQL and Database Model
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
#[graphql(name = "User")]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// The User id
    #[sea_orm(primary_key, column_type = "Text")]
    #[polar(attribute)]
    pub id:           String,

    /// The date the User was created
    pub created_at:   DateTime,

    /// The date the User was last updated
    pub updated_at:   DateTime,

    /// The User's subscriber id
    #[sea_orm(column_type = "Text")]
    #[polar(attribute)]
    pub email:        String
}

impl Default for Model {
    fn default() -> Self {
        Self {
            id:           String::default(),
            created_at:   Utc::now().naive_utc(),
            updated_at:   Utc::now().naive_utc(),
            email:        String::default(),
        }
    }
}

// The User GraphQL type is the same as the database Model
pub type User = Model;


// time to learn how relations work w/ sea_orm
// https://www.sea-ql.org/SeaORM/docs/index/

/// A wrapper around `Option<User>` to enable the trait implementations below
pub struct UserOption(pub Option<User>);

impl From<Option<Model>> for UserOption {
    fn from(data: Option<Model>) -> UserOption {
        UserOption(data)
    }
}

impl From<UserOption> for Option<User> {
    fn from(user: UserOption) -> Option<User> {
        user.0
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}