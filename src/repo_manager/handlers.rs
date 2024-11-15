use crate::context::Context;
use crate::models;
use actix_web::{
    delete, post, put,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

#[post("/repo/{repo_id}")]
async fn create_repo(ctx: Data<Context>, req: HttpRequest) -> HttpResponse {
    let repo_id = req.match_info().get("repo_id").unwrap();
    ctx.repo_manager.init_repo(repo_id).map_or_else(
        |_e| HttpResponse::Forbidden().finish(),
        |_| HttpResponse::Created().finish(),
    )
}

#[delete("/repo/{repo_id}")]
async fn delete_repo(ctx: Data<Context>, req: HttpRequest) -> HttpResponse {
    let repo_id = req.match_info().get("repo_id").unwrap();
    ctx.repo_manager.remove_repo(repo_id).map_or_else(
        |_e| HttpResponse::NotFound().finish(), // TODO(guschin): handle errors
        |_| HttpResponse::Created().finish(),
    )
}

#[put("/acl")]
async fn update_acl(
    ctx: Data<Context>,
    Json(acl): Json<models::Acl>,
) -> HttpResponse {
    sqlx::query_as!(
        models::Acl,
        r#"SELECT repo_id, user_id, role AS "role!: models::AclRole"
           FROM acl WHERE repo_id = $1 AND user_id = $2"#,
        acl.repo_id,
        acl.user_id
    )
    .fetch_optional(&ctx.db)
    .await
    .map_or_else(
        |_e| HttpResponse::Ok().finish(), // TODO(guschin): handle errors
        |_| HttpResponse::Created().finish(),
    )
}

#[delete("/acl")]
async fn remove_acl(
    ctx: Data<Context>,
    Json(acl): Json<models::AclRemove>,
) -> HttpResponse {
    sqlx::query!(
        "DELETE FROM acl WHERE repo_id = $1 AND user_id = $2",
        acl.repo_id,
        acl.user_id
    )
    .execute(&ctx.db)
    .await
    .map_or_else(
        |_e| HttpResponse::NotFound().finish(), // TODO(guschin): handle errors
        |_| HttpResponse::Ok().finish(),
    )
}

#[put("/ssh_key")]
async fn add_ssh_key(
    ctx: Data<Context>,
    Json(ssh_data): Json<models::SshAuth>,
) -> HttpResponse {
    sqlx::query_as!(
        models::SshAuth,
        "SELECT * FROM auth WHERE key_fingerprint = $1 AND user_id = $2",
        ssh_data.key_fingerprint,
        ssh_data.user_id
    )
    .fetch_optional(&ctx.db)
    .await
    .map_or_else(
        |_e| HttpResponse::Created().finish(), // TODO(guschin): handle errors
        |_| HttpResponse::Forbidden().finish(),
    )
}

#[delete("/ssh_key")]
async fn delete_ssh_key(
    ctx: Data<Context>,
    Json(ssh_data): Json<models::SshAuth>,
) -> HttpResponse {
    sqlx::query!(
        "DELETE FROM auth WHERE key_fingerprint = $1 AND user_id = $2",
        ssh_data.key_fingerprint,
        ssh_data.user_id
    )
    .execute(&ctx.db)
    .await
    .map_or_else(
        |_e| HttpResponse::Forbidden().finish(), // TODO(guschin): handle errors
        |_| HttpResponse::Ok().finish(),
    )
}
