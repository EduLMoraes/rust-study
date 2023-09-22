use rocket::{Route, get, post, http::Status};
use rocket::response::{content::Html, Redirect, status};
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
    let context = Context::new();

    let tera = Tera::new("./src/modules/templates/*.tera").expect("Erro ao carregar templates");
    let rendered = tera.render("index.tera", &context).expect("Erro ao renderizar template");

    Html(rendered)
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
            let db_librarians = user_session.search_user();
            let db_books = user_session.search_book();

            let mut context = Context::new();

            match db_books{
                Ok(books) => context.insert("books", &books),
                Err(_error_data) => ()
            }

            match db_librarians{
                Ok(librarians) => context.insert("librarians", &librarians),
                Err(_error_data) => ()
            }
        
        
            context.insert("name", &user_session.name);
            context.insert("surname", &user_session.surname);
        
            let tera = Tera::new("./src/modules/templates/view/admin/*.tera").expect("Erro ao carregar templates");
            let rendered = tera.render("home.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}
#[get("/home/admin/librarians")]
fn get_librarians_admin(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){
            let db_librarians = user_session.search_user();

            let mut context = Context::new();

            match db_librarians{
                Ok(librarians) => context.insert("librarians", &librarians),
                Err(_error_data) => ()
            }
        
            context.insert("name", &user_session.name);
            context.insert("surname", &user_session.surname);
        
            let tera = Tera::new("./src/modules/templates/view/admin/*.tera").expect("Erro ao carregar templates");
            let rendered = tera.render("librarians.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}
#[get("/home/admin/books")]
fn get_books_admin(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){
            let db_books = user_session.search_book();

            let mut context = Context::new();

            match db_books{
                Ok(books) => context.insert("books", &books),
                Err(_error_data) => ()
            }
            context.insert("name", &user_session.name);
            context.insert("surname", &user_session.surname);
        
            let tera = Tera::new("./src/modules/templates/view/admin/*.tera").expect("Erro ao carregar templates");
            let rendered = tera.render("books.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}



#[put("/add_book", data = "<form>")]
fn add_book(form: Form<BookForm>) -> status::Custom<Result<String, String>>{
    let success_response = format!("Livro '{}' adicionado com sucesso!", form.title);
    status::Custom(Status::Ok, Ok(success_response))
}



#[get("/home/librarian")]
fn librarian(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
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



#[get("/home/user")]
fn user(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "3".to_string(){
            Ok(Html(include!("../templates/view/user/home.html").to_string()))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}



#[get("/exit")]
fn user_exit(mut cookies: Cookies) -> Result<Redirect, Redirect>{
    cookies.remove_private(Cookie::named("user_session"));

    Ok(Redirect::to("/"))
}



pub fn routes()-> Vec<Route>{
    routes![
            index, login, redirect_user,
            admin, librarian, user,
            user_exit, get_librarians_admin, get_books_admin,
            add_book
        ]
}