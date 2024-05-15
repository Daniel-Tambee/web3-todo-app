pub mod todo;
pub mod user;

pub trait IModel<'a> {
    const ID: &'a str;
    const CREATED_AT: &'a str;
    const UPDATED_AT: &'a str;
    fn find_by_id(&self) -> &Self {
        return self;
    }
}
