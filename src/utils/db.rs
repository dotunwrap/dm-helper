use crate::Context;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub fn init_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not connect to database")
}

pub fn get_conn(ctx: Context<'_>) -> PooledConnection<ConnectionManager<PgConnection>> {
    ctx.data().db_pool.get().expect("Failed to get connection")
}
