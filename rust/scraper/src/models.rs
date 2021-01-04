#[derive(Queryable)]
pub struct Trope {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct MediaType {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Media {
    pub id: i32,
    pub name: String,
    pub media_type_id: i32,
}

#[derive(Queryable)]
pub struct TropeEntry {
    pub id: i32,
    pub media_id: i32,
    pub trope_id: i32,
}