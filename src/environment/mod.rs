/*
 * @Description: 
 * @Version: 1.0
 * @Autor: Sean
 * @Date: 2023-03-04 18:58:57
 * @LastEditors: Sean
 * @LastEditTime: 2023-03-05 00:16:00
 */
use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use sqlx::postgres::PgPool;
use warp::Filter;

use argon::Argon;

mod argon;

#[derive(Clone, Debug)]
pub struct Environment {
    db_pool: PgPool,
    config: Args,
    argon: Argon,
}

#[derive(Clone, Parser, Debug)]
#[clap(
name = "demo-api",
rename_all = "kebab-case",
rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(short, long)]
    debug: bool,

    #[clap(required = true, short = 'D', long)]
    database_url: String,

    #[clap(required = true, long)]
    jwt_secret: String,
    #[clap(required = true, long)]
    argon_secret: String,
    #[clap(long)]
    argon_iterations: Option<u32>,
    #[clap(long)]
    argon_memory_size: Option<u32>,

    #[clap(default_value = "0.0.0.0:8080")]
    pub host: SocketAddr,
}

impl Environment {
    pub async fn new() -> anyhow::Result<Self> {
        let args = Args::parse();
        let Args {
            database_url,
            ..
        } = &args;

        let db_pool = PgPool::connect(database_url).await?;
        let argon = Argon::new(&args);
        Ok(Self {
            db_pool,
            config: args,
            argon,
        })
    }

    pub fn db(&self) -> &PgPool {
        &self.db_pool
    }

    pub fn config(&self) -> &Args { &self.config }

    pub fn argon(&self) -> &Argon { &self.argon }
}

pub fn with_env(env: Environment) -> impl Filter<Extract=(Environment, ), Error=Infallible> + Clone {
    warp::any().map(move || env.clone())
}
