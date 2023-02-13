use anyhow::Result;
use async_graphql::{
    dataloader::Loader,
    FieldError,
    MaybeUndefined::{Null, Undefined, Value},
};
use async_trait::async_trait;
// #[cfg(test)]
// use mockall:automock;
use sea_orm::{
    entity::*,
    query::*,
    DatabaseConnection,
    EntityTrait
};
use std::{collections::HashMap, sync::Arc};

use super::{
    model::{self, User, UserOption},
    mutations::{CreateUserInput}
};

/// A UsersService applies business logic to a dynamic UsersRepository implementation
#[async_trait]
pub trait UsersServiceTrait: Sync + Send {
    /// Get `User` by ID
    async fn get(&self, id: &str) -> Result<Option<User>>;

    /// Get a list of `User` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<User>>;

    /// Get `User` by email
    async fn get_by_email(&self, email: &str) -> Result<Option<User>>;

    /// Create a `User` with the given information
    // TODO: will more than just `email` work here?
    async fn create(&self, email: &str) -> Result<User>; // should this be Result<Option<User>> if this fails or does Result cover that?

    /// Delete an existing `User`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `UsersServiceTrait` implementation
pub struct UsersService {
    /// The SeaORM database connection
    db: Arc<DatabaseConnection>,
}

impl UsersService {
    /// Create a new `UsersService` instance
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl UsersServiceTrait for UsersService {
    async fn get(&self, id: &str) -> Result<Option<User>> {
        let user = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<User>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let users = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(users)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = model::Entity::find()
                .filter(model::Column::Email.eq(email.to_owned()))
                .one(&*self.db).await?.into();

        Ok(user)
    }

    async fn create(&self, email: &str) -> Result<User> {
        let user = model::ActiveModel {
            email: Set(email.to_owned()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(user)
          
        
        // let user = model::ActiveModel {
        //     email: Set(input.email.clone()),
        //     ..Default::default()
        // };

        // let res = User::insert(user).exec(&*self.db).await?;

        // Ok(res) // maybe should be `user`?


        // let happy_bakery = bakery::ActiveModel {
        //     name: ActiveValue::Set("Happy Bakery".to_owned()),
        //     profit_margin: ActiveValue::Set(0.0),
        //     ..Default::default()
        // };
        // let res = Bakery::insert(happy_bakery).exec(db).await?;
    }

    // async fn update(&self, id: &str, input: &UpdateUserInput, with_roles: &bool) -> Result<User> {
    //     let query = model::Entity::find_by_id(id.to_owned());

    //     let (user, roles) = if *with_roles {
    //         query
    //             .find_with_related(role_grant_model::Entity)
    //             .all(&*self.db)
    //             .await?
    //             .first()
    //             .map(|t| t.to_owned())
    //     } else {
    //         query
    //             .one(&*self.db)
    //             .await?
    //             .map(|u| (u, vec![]))
    //     }.ok_or_else(|| anyhow!("Unable to find User with id: {}", id))?;

    //     let mut user: model::ActiveModel = user.into();

    //     if let Some(display_name) = &input.display_name {
    //         user.display_name = Set(display_name.clone());
    //     }

    //     match &input.display_name {
    //         Undefined => (),
    //         Null => profile.display_name = Set(None),
    //         Value(value) => profile.display_name = Set(Some(value.clone()))
    //     }

    //     match &input.picture {
    //         Undefined => (),
    //         Null => profile.picture = Set(None),
    //         Value(value) => profile.picture = Set(Some(value.clone()))
    //     }

    //     let mut updated = user.update(&*self.db).await?;

    //     // Add back the roleGrants from above
    //     updated.roles = roles;

    //     Ok(updated)
    // }

    // async fn update(&self, id: &str, input: &UpdateUserInput) -> Result<User> {
    //     let query = model::Entity::find_by_id(id.to_owned())
    //             .one(&*self.db)
    //             .await?
    //             .ok_or_else(|| anyhow!("Unable to find User with id: {}", id))?;

    //     let mut user: model::ActiveModel = query.into();

    //     if let Some(display_name) = &input.display_name {
    //         user.display_name = Set(display_name.clone());
    //     }

    //     match &input.display_name {
    //         Undefined => (),
    //         Null => input.display_name = Set(None),
    //         Value(value) => input.display_name = Set(Some(value.clone()))
    //     }

    //     match &input.picture {
    //         Undefined => (),
    //         Null => input.picture = Set(None),
    //         Value(value) => input.picture = Set(Some(value.clone()))
    //     }

    //     let mut updated = user.update(&*self.db).await?;

    //     Ok(updated)
    // }

    async fn delete(&self, id: &str) -> Result<()> {
        let user = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find User with id: {}", id))?;

        let _result = user.delete(&*self.db).await?;

        Ok(())
    }
}

/// A dataloader for `User` instances
pub struct UserLoader {
    /// the SeaORM database connection
    locations: Arc<dyn UsersServiceTrait>,
}

/// The default implementation for the `UserLoader`
impl UserLoader {
    /// Create a new instance
    pub fn new(locations: &Arc<dyn UsersServiceTrait>) -> Self {
        Self {
            locations: locations.clone(),
        }
    }
}

#[async_trait]
impl Loader<String> for UserLoader {
    type Value = User;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let locations = self.locations.get_by_ids(keys.into()).await?;

        Ok(locations
            .into_iter()
            .map(|location| (location.id.clone(), location))
            .collect())
    }
}
