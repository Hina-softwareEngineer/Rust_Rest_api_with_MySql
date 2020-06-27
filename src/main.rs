#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive; //
#[macro_use]
extern crate rocket_contrib; // maintain cors (attach)
#[macro_use]
extern crate lazy_static; // store coming data

#[macro_use]
extern crate rocket_cors; // originable

use mysql::prelude::*;
use mysql::*;
use rocket::request::Form;
use rocket::response::content::Html;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;

use rocket::http::Method;
use rocket_contrib::json::{Json, JsonValue}; // store in json format
use std::collections::HashMap; // store data in hashmap
use std::sync::{Arc, Mutex}; // capture data coming through lazy statics // Html Attributes (Http methods get, post)

// two different platforms
use rocket_cors::{
    AllowedHeaders, // wo kiya data le k aa raha hai kahan say aa rahi hai,
    AllowedOrigins,
    Cors,
    CorsOptions, // headers tells from where the request came
    Error,
};

use rocket::State; // tells about server condition

type ID = usize; // declaring globally
#[derive(Debug, PartialEq, Eq, Deserialize)] // data in the form of bytes, deserialization
struct Message {
    id: ID,
    contents: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Student {
    sid: ID,
    name: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://127.0.0.1:5500/class_06/index.html",
        "http://rust-rest-api.surge.sh/", // allow request from these
                                          // allow from local machine
    ]);
    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true, // without user name and password
        ..Default::default()
    }
    .to_cors() // convert to cross origin
    .expect("Error while building the Cros")
}

#[get("/")]
fn hello() -> JsonValue {
    let mut data = fetch();

    data
}



#[put("/update", data = "<user_input>")]
fn edit(user_input: Json<Student>, map: State<'_, MessageMap>) -> JsonValue{
    let res: Student = user_input.into_inner();
    update(res);
    json!({"status":"okay"})
}

// Mutex for real time store data on server.
type MessageMap = Mutex<HashMap<ID, Option<String>>>;
#[post("/add", data = "<user_input>")]
fn helloPost(user_input: Json<Student>, map: State<'_, MessageMap>) {
    println!("{:?}", user_input.0.name);
    println!("{:?}", user_input.0.email);
    println!("{:?}", user_input.0.age);

    let res: Student = user_input.into_inner();
    insert(res);
}



fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![hello, helloPost, edit])
        .attach(make_cors())
        .manage(Mutex::new(HashMap::<ID, Option<String>>::new()))
}

fn main() {
    rocket().launch();
}

fn insert(student: Student) {
    let pool =
        Pool::new("mysql://sql12351095:CPC85WHpBn@sql12.freemysqlhosting.net:3306/sql12351095")
            .unwrap();

    let mut conn = pool.get_conn().unwrap();
    let students = vec![student];

    let a = conn
        .exec_batch(
            r"INSERT INTO student ( name, email, age)
          VALUES ( :name, :email, :age)",
            students.iter().map(|p| {
                params! {
                    "name" => &p.name,
                    "email" => &p.email,
                    "age"=>&p.age
                }
            }),
        )
        .unwrap();
    println!("a value is  : {:?}", a);
}

fn fetch() -> JsonValue {
    let pool =
        Pool::new("mysql://sql12351095:CPC85WHpBn@sql12.freemysqlhosting.net:3306/sql12351095")
            .unwrap();

    let mut conn = pool.get_conn().unwrap();
    let selected_payments = conn
        .query_map(
            "SELECT sid, name, email, age from student",
            |(sid, name, email, age)| Student {
                sid,
                name,
                email,
                age,
            },
        )
        .unwrap();

    json!(selected_payments)
}

fn update(student: Student) {
    let pool =
        Pool::new("mysql://sql12351095:CPC85WHpBn@sql12.freemysqlhosting.net:3306/sql12351095")
            .unwrap();
    let mut conn = pool.get_conn().unwrap();

    let students = vec![student];

    conn.exec_batch(
        r"UPDATE student 
        set
        name=:name,
        email=:email,
        age=:age 
        where sid=:sid",
        students.iter().map(|p| {
            params! {
                "sid" => p.sid,
                "name" => &p.name,
                "email" => &p.email,
                "age"=>&p.age
            }
        }),
    )
    .unwrap();

    println!("updated successfully");
}



fn delete(student: Student) {
    let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing").unwrap();

    let mut conn = pool.get_conn().unwrap();
    let students = vec![student];

    conn.exec_batch(
        r"delete from student 
        where sid=:sid",
        students.iter().map(|p| {
            params! {
                "sid" => p.sid,
            }
        }),
    )
    .unwrap();
    println!("deleted successfully");
}