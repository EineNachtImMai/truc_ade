use crate::networking::{
    ade_api_handling::{get_free_rooms_calendar_list, get_zik_rooms},
    request_handling::serve,
};
// NOTE: The ADE cal goes from 6h to 21h

pub async fn serve_free_rooms() {
    let zik_rooms;
    let free_rooms;

    loop {
        match get_zik_rooms().await {
            Ok(rooms) => {
                zik_rooms = rooms;
                break;
            }
            Err(e) => eprintln!("Error: couldn't get Zik rooms ({e}).\nRetrying..."),
        };
    }

    loop {
        match get_free_rooms_calendar_list().await {
            Ok(rooms) => {
                free_rooms = rooms;
                break;
            }
            Err(e) => eprintln!("Error: couldn't get free rooms ({e}).\nRetrying..."),
        }
    }

    serve(zik_rooms, free_rooms).await;
}
