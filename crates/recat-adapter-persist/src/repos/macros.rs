#[macro_export]
macro_rules! impl_repo {
    ($entity:ty => $repo:ty; $model:ty => $table:ident) => {
        #[async_trait::async_trait]
        impl reddd::domain::ReadRepo for $repo {
            type Entity = $entity;

            async fn get(
                &self,
                key: &<Self::Entity as reddd::domain::Entity>::Key,
            ) -> reddd::domain::error::RepoResult<Self::Entity> {
                use $crate::error::DieselResultExt;
                use $crate::models::Model;
                use $crate::schema::$table::dsl::*;
                use diesel::QueryDsl;
                use diesel_async::RunQueryDsl;
                use reddd::domain::ValueType;

                let mut conn = self.0.get().await?;

                let item = $table
                    .find(key.as_inner())
                    .first::<$model>(conn.as_mut())
                    .await
                    .into_repo_result()?
                    .into_entity();

                Ok(item)
            }

            async fn get_page(
                &self,
                params: reddd::domain::Pagination<Self::Entity>,
            ) -> reddd::domain::error::RepoResult<Vec<Self::Entity>> {
                use $crate::error::DieselResultExt;
                use $crate::schema::$table::dsl;
                use diesel::prelude::*;
                use diesel::QueryDsl;
                use diesel_async::RunQueryDsl;
                use reddd::domain::ValueType;

                let mut conn = self.0.get().await?;

                let items = dsl::$table
                    .filter(
                        dsl::created_at
                            .gt(params.before_timestamp)
                            .and(dsl::id.gt(params.before_key.as_inner())),
                    )
                    .get_results::<$model>(conn.as_mut())
                    .await
                    .into_repo_result()?
                    .into_iter()
                    .map($crate::models::Model::into_entity)
                    .collect();

                Ok(items)
            }

            async fn exists(
                &self,
                key: &<Self::Entity as reddd::domain::Entity>::Key,
            ) -> reddd::domain::error::RepoResult<bool> {
                use $crate::error::DieselResultExt;
                use $crate::schema::$table::dsl::*;
                use diesel::dsl::exists;
                use diesel::QueryDsl;
                use diesel_async::RunQueryDsl;
                use reddd::domain::ValueType;

                let mut conn = self.0.get().await?;

                let result =
                    diesel::select(exists($table.find(key.as_inner())))
                        .get_result(conn.as_mut())
                        .await
                        .into_repo_result()?;

                Ok(result)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mutable_repo {
    ($entity:ty => $repo:ty; $model:ty => $table:ident) => {
        #[async_trait::async_trait]
        impl reddd::domain::WriteRepo for $repo {
            type Entity = $entity;

            async fn add(
                &self,
                item: Self::Entity,
            ) -> reddd::domain::error::RepoResult<Self::Entity> {
                use $crate::error::DieselResultExt;
                use $crate::models::Model;
                use $crate::schema::$table::dsl::*;
                use diesel_async::RunQueryDsl;

                let mut conn = self.0.get().await?;

                let model = diesel::insert_into($table)
                    .values(&<$model>::from_entity(item))
                    .get_result::<$model>(conn.as_mut())
                    .await
                    .into_repo_result()?;

                Ok(model.into_entity())
            }

            async fn update(
                &self,
                mut item: Self::Entity,
            ) -> reddd::domain::error::RepoResult<Self::Entity> {
                use $crate::error::DieselResultExt;
                use $crate::models::Model;
                use $crate::schema::$table::dsl::*;
                use diesel_async::RunQueryDsl;
                use reddd::domain::MutableEntity;

                item.touch();

                let model = <$model>::from_entity(item);
                let mut conn = self.0.get().await?;

                diesel::update($table)
                    .set(&model)
                    .execute(conn.as_mut())
                    .await
                    .into_repo_result()?;

                Ok(model.into_entity())
            }

            async fn remove(
                &self,
                key: &<Self::Entity as reddd::domain::Entity>::Key,
            ) -> reddd::domain::error::RepoResult<()> {
                use $crate::error::DieselResultExt;
                use $crate::schema::$table::dsl::*;
                use diesel::QueryDsl;
                use diesel_async::RunQueryDsl;
                use reddd::domain::ValueType;

                let mut conn = self.0.get().await?;

                diesel::delete($table.find(key.as_inner()))
                    .execute(conn.as_mut())
                    .await
                    .into_repo_result()?;

                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_full_repo {
    ($entity:ty => $repo:ty; $model:ty => $table:ident) => {
        $crate::impl_repo! { $entity => $repo; $model => $table }
        $crate::impl_mutable_repo! { $entity => $repo; $model => $table }
    };
}
