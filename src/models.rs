#[derive(Queryable)]
pub struct UniformResource {
    pub id: i32,
    pub uniform_resource_locator: String,
    pub uniform_resource_identifier: String,
    pub hits: u64,
}
