use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SupabaseConfig {
    pub supabase_url_storage: String,
    pub supabase_api_key: Option<String>,
}

impl Default for SupabaseConfig {
    fn default() -> Self {
        envy::from_env::<SupabaseConfig>().unwrap()
    }
}
