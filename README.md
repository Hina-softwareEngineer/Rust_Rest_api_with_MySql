# ✨ RUST REST API✳️✳️ WITH MYSQL DATABASE (CRUD) 🔥🔥 USING ROCKET :rocket: :rocket:

⚡️ Demo link : http://rust-rest-api.surge.sh/
<br>
⚡️ Rust-Server Link : https://rocket-restapi-crud.herokuapp.com/

#### - Rocket Requests (with Cors)

1. Get Request
2. Post Request
3. Put Request
4. Delete Request

## ✨ Where I host MySql Database for testing?

Link : https://www.freemysqlhosting.net/
<br>
It gives you free 5MB.

### ✨ Data Fetching (Get Request)

```

#[get("/")]
fn getRequest() -> JsonValue {
    let mut data = fetch();

    data
}

fn fetch() -> JsonValue {
    let pool =
        Pool::new("mysql://username:password@host:3306/database_name")
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

```

### ✨ Insertion of Data (Post Request)

```
#[post("/add", data = "<user_input>")]
fn helloPost(user_input: Json<Student>, map: State<'_, MessageMap>) -> JsonValue {

    let res: Student = user_input.into_inner();
    let result = insert(res);

    result
}

fn insert(student: Student) -> JsonValue {
    let pool =
        Pool::new("mysql://username:password@host:3306/database_name")
            .unwrap();

    let mut conn = pool.get_conn().unwrap();
    let students = vec![student];

    let b = conn
        .exec_batch(
            r"INSERT INTO student (name, email, age)
          VALUES (:name, :email, :age)",
            students.iter().map(|p| {
                params! {
                    "name" => &p.name,
                    "email" => &p.email,
                    "age"=>&p.age
                }
            }),
        )
        .unwrap();

    let c = conn.last_insert_id();    // gives the last inserted data id
    println!("c value is : {:?}", c);
    json!({ "id": c })
}


```

### ✨ Updation of Data (Put Request)

```

#[put("/update", data = "<user_input>")]
fn edit(user_input: Json<Student>, map: State<'_, MessageMap>) -> JsonValue {
    let res: Student = user_input.into_inner();
    update(res);
    json!({"status":"okay"})
}


fn update(student: Student) {
    let pool =
        Pool::new("mysql://username:password@host:3306/database_name")
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


```

### ✨ Deletion of Data (Delete Request)

```

#[delete("/delete/<id>")]
fn deleted(id: i32) {
    delete(id);
}

fn delete(id1: i32) {
    let pool =
        Pool::new("mysql://username:password@host:3306/database_name")
            .unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.exec_drop(
        r"delete from student
        where sid=:sid",
        params! {
            "sid"=> id1,
        },
    )
    .unwrap();
    println!("deleted successfully {:?}", id1);
}

```

> ⚡️ **Note** : I am adding my frontend Code in this repo, This code has no link with heroku. First I pushed my rust code to heroku, then I add my frontend code so that all my code placed in one repo.

##### For Heroku Deployment : https://github.com/Hina-softwareEngineer/Rust_Server

**Moreover, If you want to make Pull Requests, You are wellcome. ✌️ ✌️**
