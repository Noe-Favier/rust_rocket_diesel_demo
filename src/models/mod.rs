use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Liste {
    pub id: i32,
    pub libelle: String,
}
