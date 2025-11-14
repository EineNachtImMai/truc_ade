use crate::networking::{
    ade_api_handling::{get_free_rooms_calendar_list, get_zik_rooms},
    request_handling::serve,
};
// NOTE: The ADE cal goes from 6h to 21h

pub async fn serve_free_rooms() {
    serve(get_zik_rooms().await, get_free_rooms_calendar_list().await).await;
}
