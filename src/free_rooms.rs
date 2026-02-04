use crate::networking::request_handling::serve;

use crate::caching::cal_caching::{init_cal_last_update_time, init_resource_last_update_time};
// NOTE: The ADE cal goes from 6h to 21h

pub async fn serve_free_rooms() {
    loop {
        match init_resource_last_update_time() {
            Ok(_) => {
                break;
            }
            Err(e) => {
                tracing::error!("Error: couldn't initialize source cache: ({e}).\nRetrying...")
            }
        };
    }

    loop {
        match init_cal_last_update_time() {
            Ok(_) => {
                break;
            }
            Err(e) => {
                tracing::error!("Error: couldn't initialize calendar cache: ({e}).\nRetrying...")
            }
        };
    }

    serve().await; // TODO: caching instead
}
