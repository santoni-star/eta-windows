use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredInteraction {
    pub id: i64,
    pub user_message: String,
    pub assistant_response: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct MemoryStore {
    pool: SqlitePool,
}

impl MemoryStore {
    pub async fn new(db_path: &Path) -> Result<Self> {
        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path.display())).await?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;

        Ok(Self { pool })
    }

    pub async fn store_interaction(&self, user_msg: &str, assistant_resp: Option<&str>) -> Result<()> {
        sqlx::query(
            "INSERT INTO interactions (user_message, assistant_response, timestamp) VALUES (?1, ?2, ?3)"
        )
        .bind(user_msg)
        .bind(assistant_resp)
        .bind(chrono::Utc::now())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_interactions(&self, limit: i64) -> Result<Vec<StoredInteraction>> {
        let rows = sqlx::query_as(
            "SELECT id, user_message, assistant_response, timestamp FROM interactions ORDER BY timestamp DESC LIMIT ?1"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn search_memory(&self, query: &str) -> Result<Vec<StoredInteraction>> {
        let rows = sqlx::query_as(
            "SELECT id, user_message, assistant_response, timestamp FROM interactions WHERE user_message LIKE ?1 OR assistant_response LIKE ?1 ORDER BY timestamp DESC LIMIT 20"
        )
        .bind(format!("%{}%", query))
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}

pub mod commands {
    use tauri::State;

    use crate::AppState;

    #[tauri::command]
    pub async fn search_memory(
        query: String,
        state: State<'_, AppState>,
    ) -> Result<Vec<crate::agent::memory::StoredInteraction>, String> {
        let agent = state.agent.lock().unwrap();
        agent.memory.search_memory(&query).await
            .map_err(|e| e.to_string())
    }
}
