use mysql_async::{Pool, PoolOpts, PoolConstraints, OptsBuilder};
use std::time::Duration;
use log::*;

#[tokio::main]
async fn main() {
    init_logger();

    let run_mariadb = true;
    let run_tidb = true;


    if run_mariadb {
        info!("===== Starting MariaDB Test =====");
        let pool = get_pool("127.0.0.1", 3306, "root", Some("password"), "db");
        run_test(pool).await;
    }

    if run_tidb {
        info!("===== Starting TiDB Test =====");
        let pool = get_pool("127.0.0.1", 4000, "root", None, "db");
        run_test(pool).await;
    }
}

async fn run_test(pool: Pool) {
    for i in 0..6 {
        let c = pool.get_conn().await.unwrap();
        info!("Iteration {} connection: id={}", i, c.id());
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Create pool
fn get_pool(host: &str, port: u16, user: &str, password: Option<&str>, database: &str) -> Pool {
    let pool_opts = PoolOpts::default().with_constraints(PoolConstraints::new(1, 10).unwrap());

    let opts = OptsBuilder::default()
        .pool_opts(pool_opts)
        .user(Some(user))
        .pass(password)
        .ip_or_hostname(host)
        .tcp_port(port)
        .db_name(Some(database));

    Pool::new(opts)
}


/// Set up logging
fn init_logger() {
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
}
