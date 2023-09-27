/// Módulo de rotas do bibliotecário.

use rocket::http::Cookies;
use rocket::response::{content::Html, Redirect};
//use tera::{Tera, Context};

use crate::router::structs::*;

#[get("/home/librarian")]
pub fn librarian(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "2".to_string(){
            Ok(Html(include!("../templates/view/librarian/home.html").to_string()))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}

