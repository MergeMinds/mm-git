use actix_web::web;

use crate::repo_manager;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(repo_manager::handlers::create_repo)
       .service(repo_manager::handlers::delete_repo)
       .service(repo_manager::handlers::update_acl)
       .service(repo_manager::handlers::remove_acl)
       .service(repo_manager::handlers::add_ssh_key)
       .service(repo_manager::handlers::delete_ssh_key);
}
