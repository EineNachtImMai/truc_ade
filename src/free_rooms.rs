use crate::networking::{ade_api_handling::get_calendar_list, request_handling::serve};
// NOTE: The ADE cal goes from 6h to 21h

pub fn serve_free_rooms() {
    serve(get_calendar_list());
}
