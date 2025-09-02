use crate::config;
use crate::dto::auth::{AuthResponse, LoginRequest, RegisterRequest, SearchMail};
use crate::entities::prelude::Users;
use crate::server::server::Server;
use crate::utils::{database, logger};
use anyhow::{anyhow, Result};
use axum::Router;
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::{ColumnTrait, NotSet};
use uuid::Uuid;
use crate::entities::users;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> AppState {
        AppState { db }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse> {
        // 检查邮箱是否已存在
        let existing = Users::find()
            .filter(users::Column::Email.eq(&req.email))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(anyhow!("Email already exists"));
        }

        // 密码加密
        let password_hash = hash(req.password, DEFAULT_COST)?;

        // 创建用户
        let user_model = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(req.email.clone()),
            password_hash: Set(password_hash),
            name: Set(req.name.clone()),
            created_at: NotSet,
        };

        let user = user_model.insert(&self.db).await?;

        Ok(AuthResponse {
            user_id: user.id,
            email: user.email,
            name: user.name,
            message: "Registration successful".to_string(),
        })
    }

    // 登录
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse> {
        // 查找用户
        let user = Users::find()
            .filter(users::Column::Email.eq(&req.email))
            .one(&self.db)
            .await?;

        let user = match user {
            Some(u) => u,
            None => return Err(anyhow!("Invalid email or password")),
        };

        // 验证密码
        if !verify(req.password, &user.password_hash)? {
            return Err(anyhow!("Invalid email or password"));
        }

        Ok(AuthResponse {
            user_id: user.id,
            email: user.email,
            name: user.name,
            message: "Login successful".to_string(),
        })
    }

    // 根据ID获取用户
    pub async fn get_user(&self, search_mail: SearchMail) -> Result<Option<users::Model>> {
        let user = Users::find()
            .filter(users::Column::Email.eq(&search_mail.mail))
            .one(&self.db)
            .await?;
        Ok(user)
    }
}

pub async fn run(router: Router<AppState>) -> Result<()> {
    logger::init();
    let db = database::init().await?;
    let state = AppState::new(db);
    let server = Server::new(config::get().server_config());
    server.start(state, router).await?;
    Ok(())
}
