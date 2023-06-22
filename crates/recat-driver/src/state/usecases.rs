use super::RecatAppState;
use recat_app::{state::UseCaseRunner, usecases::prelude::*};
use recat_core::usecases::prelude::*;
use reddd::domain::{UseCase, UseCaseHandler};

macro_rules! set_handler {
    ($usecase:ty => $handler:ty) => {
        #[async_trait::async_trait]
        impl UseCaseRunner<$usecase> for RecatAppState {
            #[inline]
            async fn run_usecase(
                &self,
                input: <$usecase as UseCase>::Input,
            ) -> Result<
                <$usecase as UseCase>::Output,
                <$usecase as UseCase>::Error,
            > {
                <$handler>::execute(input, self).await
            }
        }
    };
}

// products
set_handler!(GetProduct => GetProductHandler);
set_handler!(GetProductPage => GetProductPageHandler);
set_handler!(AddProduct => AddProductHandler);
set_handler!(UpdateProduct => UpdateProductHandler);
set_handler!(RemoveProduct => RemoveProductHandler);
