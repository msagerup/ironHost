mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;



fn main() {
    match get_user_response("What web-server are we building today?") {
        Ok(user_request) => {
            dbg!(user_request);
        }
        Err(e) => eprintln!("An error occurred: {}", e),
    } 
}