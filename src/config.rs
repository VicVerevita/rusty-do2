pub struct AppState {
    pub is_authenticated: bool,
    pub user_id: Option<uuid::Uuid>,
}
