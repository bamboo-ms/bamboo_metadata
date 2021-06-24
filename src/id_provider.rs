pub trait IdProvider {
    // The identifier or name of the database the identifier
    // is used for
    fn database_id(&self) -> String;
    // The unique identifier for the entity in that database
    fn database_value(&self) -> String;
}
