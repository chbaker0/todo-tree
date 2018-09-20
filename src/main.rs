#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod todo_list;
mod todo_list_store;

use rkt::http;
use rkt::response::status;
use rkt::response::Responder;
use rocket as rkt;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use todo_list::TodoList;
use todo_list_store::*;

/// Represents an operation that can fail. This is structurally
/// similar to `Option<T>` but different semantically. Additionally,
/// it allows a `String` to be returned as a failure message.
///
/// This implements a `Responder` that returns HTTP 500 upon `Fail`.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Failable<T> {
    Succ(T),
    Fail(String),
}

impl<'r, T: Responder<'r>> Responder<'r> for Failable<T> {
    fn respond_to(self, request: &rkt::Request) -> Result<rkt::Response<'r>, http::Status> {
        match self {
            Failable::Succ(x) => x.respond_to(request),
            Failable::Fail(s) => {
                status::Custom(http::Status::InternalServerError, s).respond_to(request)
            }
        }
    }
}

/// Contains the server's state used in the various handlers. Mutable
/// fields must be in `Mutex`es since Rocket is multithreaded.
struct ServerState {
    todo_list_store: Mutex<InMemoryStore>,
}

//Todo: when the next version of rocket comes out it will probably have better support for serving
//static files, use the method mentioned here: https://github.com/SergioBenitez/Rocket/issues/239
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("todo-tree-ui/dist/todo-tree-ui/index.html").ok()
}

#[get("/<file..>", rank = 10)] // use rank here to allow other api endpoint available as well
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("todo-tree-ui/dist/todo-tree-ui/").join(file)).ok()
}

/// HTTP handler for deleting lists.
#[delete("/lists/<id>", format = "application/json")]
fn delete_list(state: rkt::State<ServerState>, id: u64) -> Option<()> {
    let mut list_store = state.todo_list_store.lock().unwrap();
    list_store.delete(TodoListId(id)).ok()
}

/// HTTP handler for modifying lists. Currently, it just sets the title.
#[put("/lists/<id>", format = "application/json", data = "<title>")]
fn update_list(state: rkt::State<ServerState>, id: u64, title: String) -> Option<()> {
    let todo_list_id = TodoListId(id);
    let mut list_store = state.todo_list_store.lock().unwrap();
    let todo_list = TodoList {
        title: title.to_string(),
        entries: Default::default(),
    };
    list_store.update(todo_list_id, &todo_list).ok()
}

/// HTTP handler for retrieving lists. Since we are only setting the
/// title, this only returns the title.
#[get("/lists/<id>", format = "application/json")]
fn get_list(state: rkt::State<ServerState>, id: u64) -> Option<String> {
    let todo_list_id = TodoListId(id);
    let list_store = state.todo_list_store.lock().unwrap();
    list_store.getone(todo_list_id).map(|t| t.title).ok()
}

/// HTTP handler for creating lists. Currently, it just sets the
/// title. Returns the ID as a string.
#[post("/lists", format = "application/json", data = "<title>")]
fn create_list(state: rkt::State<ServerState>, title: String) -> Json<String> {
    let todo_list = TodoList {
        title: title.to_string(),
        entries: Default::default(),
    };
    let mut list_store = state.todo_list_store.lock().unwrap();
    match list_store.create(&todo_list) {
        Ok(x) => Json(format!("Created Todo List with id {}.", x.0)),
        Err(_) => Json("Failed!".to_string()),
    }
}

///Create rocket instance
fn rocket() -> rocket::Rocket {
    rkt::ignite()
        .manage(ServerState {
            todo_list_store: Mutex::new(InMemoryStore::new()),
        }).mount(
            "/",
            routes![
                create_list,
                get_list,
                update_list,
                delete_list,
                files,
                index
            ],
        )
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_create() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        let response = client
            .post("/lists")
            .body("title=abc")
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        //add a list
        client
            .post("/lists")
            .body("title=abc")
            .header(ContentType::JSON)
            .dispatch();
        let response1 = client
            .get(format!("/lists/{}", 0))
            .header(ContentType::JSON)
            .dispatch();
        let response2 = client
            .get(format!("/lists/{}", 9))
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response1.status(), Status::Ok);
        assert_eq!(response2.status(), Status::NotFound);
    }

    #[test]
    fn test_update() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        client
            .post("/lists")
            .body("title=abc")
            .header(ContentType::JSON)
            .dispatch();
        let response1 = client
            .put(format!("/lists/{}", 0))
            .body("title=xyz")
            .header(ContentType::JSON)
            .dispatch();
        let response2 = client
            .put(format!("/lists/{}", 9))
            .body("title=xyz")
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response1.status(), Status::Ok);
        assert_eq!(response2.status(), Status::NotFound);
    }

    #[test]
    fn test_delete() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        client
            .post("/lists")
            .body("title=abc")
            .header(ContentType::JSON)
            .dispatch();
        client
            .delete(format!("/lists/{}", 0))
            .body("title=xyz")
            .header(ContentType::JSON)
            .dispatch();
        let response = client
            .get(format!("/lists/{}", 0))
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}
