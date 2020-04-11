use crate::database::schema::elections;

/// Representation of an Election already stored inside the database
#[derive(Queryable)]
pub struct Election {
    pub id: i32,
    /// Initial datetime of the election
    pub init_date: chrono::NaiveDateTime,
    /// Final datetime of the election
    pub end_date: chrono::NaiveDateTime,
    /// Datetime in which the entry was created
    pub created_on: chrono::NaiveDateTime,
}

/// Struct used to insert a new election into the database
#[derive(Insertable)]
#[table_name = "elections"]
pub struct InsertableElection {
    pub id: i32,
    /// Initial datetime of the election
    pub init_date: chrono::NaiveDateTime,
    /// Final datetime of the election
    pub end_date: chrono::NaiveDateTime,
    /// Datetime in which the entry was created
    pub created_on: chrono::NaiveDateTime,
}