use crate::Context;
use mysql::*;

pub fn init_dnd_db(database_url: &str) -> Pool {
    Pool::new(
        OptsBuilder::from_opts(Opts::from_url(database_url).expect("Could not parse database URL"))
            .ssl_opts(SslOpts::default()),
    )
    .expect("Could not connect to database")
}

pub fn get_db_conn(ctx: Context<'_>) -> PooledConn {
    ctx.data().db.get_conn().expect("Failed to get connection")
}
