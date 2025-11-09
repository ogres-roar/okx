use okx::logic;
use okx::util::log::init_log;


#[tokio::main]
async fn main() {
    init_log();

    tokio::spawn(async move {
        logic::list::list().await;
    });

    logic::spot_swap::spot_swap_arbitrage().await;
}
