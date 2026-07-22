use axum::{Json, Router, routing::get};
use serde::Deserialize;

use crate::{app::AppState, auth::admin::AdminAuth, error::AppError, models::Asset, repository::{Repository}};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", get(list_assets)
        .post(create_asset).patch(update_asset))
}

#[tracing::instrument(skip_all)]
async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _: AdminAuth,
    repository: Repository, 
    Json(request) : Json<CreateAssetRequest>)
    -> Result<Json<Asset>, AppError> {
    let new_asset = repository
    .create_asset(request.name, request.unit_value)
    .await?;
    
    Ok(Json(new_asset))
} 

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _: AdminAuth,
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repository
        .update_asset(request.id, request.name, request.unit_value)
        .await?
       {
        Some(update_asset) => Ok(Json(update_asset)),
        None => Err(AppError::AssetDoesNotExist),
       }
}

#[cfg(test)]
mod tests {
use sqlx::PgPool;
use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
      let request = CreateAssetRequest {
        name: "Teste".to_string(), 
        unit_value: 100.0,
    };

      let Json(new_asset) =  create_asset(AdminAuth, db.into(), Json(request)).await.expect("Success"); 

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Teste");
        assert_eq!(new_asset.unit_value, 100.0);

        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async  fn test_list_assets(db: PgPool) {
       let Json(assets) = list_assets(db.into()).await.expect("Success");

       assert_eq!(assets.len(), 1);
       assert_eq!(assets[0].name, "Bitcoin");

       insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
      async fn test_update_asset(db: PgPool) {
      let request = UpdateAssetRequest {
        id: 1,
        name: Some("Teste de update".to_string()),
        unit_value: Some(300.0),
      };

      let Json(new_asset) =  update_asset(AdminAuth, db.into(), Json(request)).await.expect("Success"); 

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Teste de update");
        assert_eq!(new_asset.unit_value, 300.0);

        insta::assert_json_snapshot!(new_asset);
    }

}