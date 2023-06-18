use deadpool::managed::Object;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use reddd::domain::error::{RepoError, RepoResult};

use crate::config::{DataConfig, DataConfigExt};

pub(crate) type ConnectionType = AsyncPgConnection;
pub(crate) type ManagerType = AsyncDieselConnectionManager<ConnectionType>;
pub(crate) type ObjectType = Object<ManagerType>;

pub struct DbPool {
    pool: Pool<ConnectionType>,
}

impl DbPool {
    pub(crate) async fn get(&self) -> RepoResult<ObjectType> {
        match self.pool.get().await {
            Ok(conn) => Ok(conn),
            Err(err) => Err(RepoError::Other(Box::new(err))),
        }
    }

    pub fn new(config: &impl DataConfig) -> anyhow::Result<Self> {
        let mut builder = Pool::builder(ManagerType::new(config.connection_string()));

        if let Some(max) = config.pool_max_connections() {
            builder = builder.max_size(max);
        }

        if let Some(timeout) = config.pool_timeout() {
            builder = builder.recycle_timeout(Some(timeout.to_std()?));
        }

        Ok(DbPool {
            pool: builder.build()?,
        })
    }
}
