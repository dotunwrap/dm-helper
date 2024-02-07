use crate::Context;
use mysql::*;

pub fn init_dnd_db() -> Pool {
    Pool::new(
        OptsBuilder::from_opts(
            Opts::from_url(&std::env::var("DND_DATABASE_URL").expect("Database URL not found"))
                .expect("Could not parse database URL"),
        )
        .ssl_opts(SslOpts::default()),
    )
    .expect("Could not connect to database")
}

pub fn get_db_conn(ctx: Context<'_>) -> PooledConn {
    ctx.data().db.get_conn().expect("Failed to get connection")
}
