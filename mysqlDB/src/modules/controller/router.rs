use rocket::{Route, get, post};
use rocket::response::{content::Html, Redirect, Responder};
use rocket::request::Form;
use tokio::runtime;
use rocket::http::{Cookies, Cookie};
use mysql::serde_json;
use tera::{Tera, Context};

#[path = "structs.rs"]
mod structs;
use structs::*;

#[get("/")]
fn index() -> Html<String>{
    Html(include!("../templates/index.html").to_string())
}



#[path = "./login.rs"]
mod login;


#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>, mut cookies: Cookies) -> Redirect{
    let email = &form.email;
    let password = &form.password;

    let rt = runtime::Runtime::new().unwrap();
    let login = rt.block_on(login::login(email.to_string(), password.to_string()));

    match login{
        Ok(user_session) => { 
            cookies.add_private(Cookie::new("user_session", serde_json::to_string(&user_session).unwrap()));
            Redirect::to("/home")
        },
        Err(_) => Redirect::to("/")
    }
}



#[get("/home")]
fn redirect_user(mut cookies: Cookies) -> Result<Redirect, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){
            Ok(Redirect::to("/home/admin"))
        }else if user_session.permission == "2".to_string(){
            Ok(Redirect::to("/home/librarian"))
        }else {
            Ok(Redirect::to("/home/user"))
        }
    } else {
        // A sessão do usuário não existe, redirecione para a página de login
        Err(Redirect::to("/"))
    }
}



#[get("/home/admin")]
fn admin(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){

            

            let livros: Vec<Book> = vec![
                // Preencha esta lista com seus dados de livros
            ];
        

            let mut context = Context::new();
            context.insert("livros", &livros);
        
            let tera = Tera::new("./src/modules/templates/*.tera").expect("Erro ao carregar templates");
            let rendered = tera.render("home-admin.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}



#[get("/home/librarian")]
fn librarian(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "2".to_string(){
            Ok(Html(include!("../templates/home-librarian.html").to_string()))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}



#[get("/home/user")]
fn user(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "3".to_string(){
            Ok(Html(include!("../templates/home-user.html").to_string()))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}



pub fn routes()-> Vec<Route>{
    routes![
            index, login, redirect_user,
            admin, librarian, user
        ]
}