mod product;

pub(crate) use product::ProductModel;

pub(crate) trait Model {
    type Entity: reddd::domain::Entity;

    fn from_entity(entity: Self::Entity) -> Self;
    fn into_entity(self) -> Self::Entity;
}
