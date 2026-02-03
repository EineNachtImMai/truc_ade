use crate::networking::{ade_api_handling::get_zik_rooms, request_handling::serve};

use crate::caching::cal_caching::{init_cal_last_update_time, init_resource_last_update_time};
// NOTE: The ADE cal goes from 6h to 21h

pub async fn serve_free_rooms() {
    let zik_rooms;

    loop {
        match init_resource_last_update_time() {
            Ok(_) => {
                break;
            }
            Err(e) => eprintln!("Error: couldn't initialize source cache: ({e}).\nRetrying..."),
        };
    }

    loop {
        match init_cal_last_update_time() {
            Ok(_) => {
                break;
            }
            Err(e) => eprintln!("Error: couldn't initialize source cache: ({e}).\nRetrying..."),
        };
    }

    loop {
        match get_zik_rooms().await {
            Ok(rooms) => {
                zik_rooms = rooms;
                break;
            }
            Err(e) => eprintln!("Error: couldn't get Zik rooms ({e}).\nRetrying..."),
        };
    }

    /* loop {
        match get_free_rooms_calendar_list().await {
            Ok(rooms) => {
                free_rooms = rooms;
                break;
            }
            Err(e) => eprintln!("Error: couldn't get free rooms ({e}).\nRetrying..."),
        }
    } */

    serve(zik_rooms).await; // TODO: caching instead
}
