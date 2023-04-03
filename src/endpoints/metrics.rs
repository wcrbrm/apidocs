use super::prelude::*;

use crate::readers::fetch_merchant_stat;
use lazy_static::lazy_static;
use prometheus::{opts, register_int_gauge};
#[allow(unused_imports)]
use prometheus::{Encoder, Gauge, IntGauge, Opts, Registry, TextEncoder};

lazy_static! {
    pub static ref UP: IntGauge =
        register_int_gauge!(opts!("up", "Whether the server is running")).unwrap();
}

async fn to_string(conn: &mut Conn) -> String {
    let encoder = TextEncoder::new();
    // let labels = HashMap::new();
    // let sr = Registry::new_custom(Some("api".to_string()), Some(labels)).unwrap();
    let sr = Registry::new();
    sr.register(Box::new(UP.clone())).unwrap();
    UP.set(1i64);

    if let Ok(merchants) = fetch_merchant_stat(conn).await {
        for (merchant, num) in merchants {
            let gauge_opts = Opts::new("products", "for each merchant")
                .const_label("merchant", merchant.as_str());
            let gauge = Gauge::with_opts(gauge_opts).unwrap();
            gauge.set(num as f64);
            sr.register(Box::new(gauge.clone())).unwrap();
        }
    }
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&sr.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}

pub async fn handle(state: Extension<Arc<AppState>>) -> impl IntoResponse {
    let pool = state.pool.clone();
    if let Ok(mut conn) = pool.get().await {
        return to_string(&mut conn).await.into_response();
    }
    errconn().into_response()
}
