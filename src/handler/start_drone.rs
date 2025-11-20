use super::*;

pub struct StartDrone;

#[derive(Debug, Deserialize, Serialize)]
pub struct StartDroneReq {
    uuid: String,
    organization_uuid: String,
    project_uuid: String,
    sn: String,
}

impl ExecSql<StartDroneReq> for StartDrone {
    async fn handle_post(
        Extension(cfg): Extension<Arc<Config>>,
        params: Result<Json<StartDroneReq>, JsonRejection>,
    ) -> Result<Json<Value>, WebErr> {
        let Json(prms) = params?;
        let _ = log_stream(&cfg.db_path, &prms).await?;
        Ok(Json(json!({
            "success": true
        })))
    }
}

async fn log_stream(db_path: &str, req: &StartDroneReq) -> Result<i64> {
    let mut conn = SqliteConnection::connect(db_path).await?;
    let sql = r#"
        replace into sn(sn, uuid, project_uuid, organization_uuid)
        values(?,?,?,?)
        "#;
    let r = sqlx::query(sql)
        .bind(&req.sn)
        .bind(&req.uuid)
        .bind(&req.project_uuid)
        .bind(&req.organization_uuid)
        .execute(&mut conn)
        .await?;

    Ok(r.last_insert_rowid())
}
