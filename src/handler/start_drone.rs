use super::*;

pub struct StartDrone;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct StartDroneReq {
    uuid: String,
    organization_uuid: String,
    project_uuid: String,
    flight_uuid: String,
    sn: String,
    rtmp: String,
    labels: Vec<Label>,
    #[serde(rename = "tenantId")]
    tenant_id: i64,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    id: i64,
    code: String,
    name: String,
    tenant_id: i64,
    created_at: Option<String>,
    updated_at: Option<String>,
    deleted_at: Option<String>,
}

impl ExecSql<StartDroneReq> for StartDrone {
    async fn handle_post(
        Extension(cfg): Extension<Arc<Config>>,
        params: Result<Json<StartDroneReq>, JsonRejection>,
    ) -> Result<Json<Value>, WebErr> {
        let Json(prms) = params?;
        let _ = log_info(&cfg.db_path, &prms).await?;
        Ok(Json(json!({
            "success": true
        })))
    }
}

async fn log_info(db_path: &str, req: &StartDroneReq) -> Result<i64> {
    let mut conn = SqliteConnection::connect(db_path).await?;
    let sn_sql = r#"
        replace into sn(sn, uuid, project_uuid, organization_uuid)
        values(?,?,?,?)
        "#;
    let r = sqlx::query(sn_sql)
        .bind(&req.sn)
        .bind(&req.uuid)
        .bind(&req.project_uuid)
        .bind(&req.organization_uuid)
        .execute(&mut conn)
        .await?;

    let tag_sql = r#"
        insert into stream_tag(uuid, code, name)
        values(?,?,?)
        "#;
    for lb in req.labels.iter() {
        let _r = sqlx::query(tag_sql)
            .bind(&req.uuid)
            .bind(&lb.code)
            .bind(&lb.name)
            .execute(&mut conn)
            .await?;
    }

    Ok(r.last_insert_rowid())
}
