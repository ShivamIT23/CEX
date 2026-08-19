pub enum market {
    sol_usd,
    sol_eth,
    eth_usd,
    eth_sol,
    usd_sol,
    usd_eth,
}

pub enum order_action {
    BUY,
    SELL,
}

pub enum order_type {
    LIMIT,
    MARKET,
}

pub struct order {
    pub action: order_type,
    pub order_type: order_type,
    pub price: u32,
    pub qty: u32,
    market: market,
}
