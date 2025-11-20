use super::*;
use sysinfo::System;

pub struct StopDrone;

#[derive(Debug, Deserialize, Serialize)]
pub struct StopDroneReq {
    uuid: String,
}

impl ExecSql<StopDroneReq> for StopDrone {
    async fn handle_post(
        cfg: Extension<Arc<Config>>,
        params: Result<Json<StopDroneReq>, JsonRejection>,
    ) -> Result<Json<Value>, WebErr> {
        let Json(prms) = params?;
        let mut conn = SqliteConnection::connect(&cfg.db_path).await?;

        let _ = sqlx::query("delete from sn where uuid = ?")
            .bind(&prms.uuid)
            .execute(&mut conn)
            .await?;

        let sys = System::new_all();
        for p in sys.processes().values() {
            if let Some(s) = p.name().to_str()
                && s.contains(&cfg.ffmpeg_dump_name)
                && p.cmd()
                    .iter()
                    .any(|c| c.to_str().unwrap_or("").contains(&prms.uuid))
            {
                let b = p.kill();
                return Ok(Json(json!({
                    "success": b,
                })));
            }
        }

        Ok(Json(json!({
            "success": false,
        })))
    }
}
