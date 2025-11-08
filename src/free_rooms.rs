mod request_handling;
mod ade_api_handling;

use request_handling::serve;
use ade_api_handling::get_calendar_list;
// NOTE: The ADE cal goes from 6h to 21h

pub fn serve_free_rooms() {
    serve(get_calendar_list());
}
