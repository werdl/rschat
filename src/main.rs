use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::params;
use std::collections::HashMap;
use warp::{self, reply, Filter};
use thetime::{Ntp, Time};
use percent_encoding::{percent_decode_str, utf8_percent_encode, CONTROLS};
extern crate time;

#[tokio::main]
async fn main() {
    // list("test".to_string());

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let new = warp::path!("new" / String / String).and_then(handle_new_user);
    // new/<user>/<pass>

    let login = warp::path!("login" / String / String)
        .map(|user, pass| reply::json::<Vec<bool>>(&vec![valid(user, pass)]));
    // login/<user>/<pass>

    let exists_q = warp::path!("exists" / String).map(|name: String| {
        let exists_result = exists(&name.clone());
        reply::html(format!("{} exists: {}", name, exists_result))
    });
    // exists/<user>

    let post =
        warp::path!("post" / String / String / String / String).map(|user, pass, to, msg: String| {
            reply::html(if post(user, pass, percent_decode_str(&msg).decode_utf8().unwrap().to_string(), to) {
                "ERR"
            } else {
                "OK"
            })
        });
    // post/<user>/<pass>/<to>/<msg>

    let list = warp::path!("list" / String / String).map(|user: String, pass| {
        if valid(user.clone(), pass) {
            let db_conn = get_db_conn();
            let query = format!("SELECT * FROM messages WHERE to_uname = \"{}\"", user);

            let maps = match db_conn.prepare(&query) {
                Ok(mut stmt) => stmt
                    .query_map(params![], |row| {
                        let mut map = HashMap::new();
                        map.insert("from_uname", row.get::<usize, String>(1).unwrap());
                        map.insert("msg", row.get::<usize, String>(3)?);
                        map.insert("ts", row.get::<usize, String>(4)?);
                        Ok(map)
                    })
                    .map(|result| result.collect::<Result<Vec<_>, _>>()),

                Err(_) => {
                    return reply::json(&vec!["Database query failed"]);
                }
            };

            match maps {
                Ok(maps) => reply::json(&maps.unwrap()),
                Err(_) => reply::json(&vec!["Database query failed"]),
            }
        } else {
            reply::json::<Vec<&str>>(&vec!["BADUNAME"])
        }
    });

    warp::serve(hello.or(new).or(login).or(exists_q).or(post).or(list))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

async fn handle_new_user(
    name: String,
    pass: String,
) -> Result<reply::Html<&'static str>, warp::Rejection> {
    let hashed_pass = hash(&pass, DEFAULT_COST).expect("Failed to hash password");
    let db_conn = get_db_conn();

    if db_conn
        .execute(
            "INSERT INTO users (uname, pass_hash) VALUES (?, ?)",
            params![name, hashed_pass],
        )
        .is_err()
    {
        Ok(reply::html("ERR"))
    } else {
        Ok(reply::html("OK"))
    }
}

fn post(name: String, pass: String, msg: String, to: String) -> bool {
    if valid(name.clone(), pass) {
        let db_conn = get_db_conn();
        db_conn
            .execute(
                "INSERT INTO messages (from_uname, to_uname, msg, ts) VALUES (?, ?, ?, ?) ",
                params![
                    name,
                    to,
                    msg,
                    Ntp::now().unix().to_string()
                ],
            )
            .is_err()
    } else {
        false
    }
}

fn valid(name: String, pass: String) -> bool {
    let db_conn = get_db_conn();
    let hashed_pass: String = db_conn
        .query_row(
            "SELECT pass_hash FROM users WHERE uname = ?1",
            params![name],
            |row| row.get(0),
        )
        .unwrap_or_default();

    verify(&pass, &hashed_pass).unwrap_or(false)
}
// -> Vec<HashMap<String, String>>
// fn list(name: String) {
//     println!("list");

//     let db_conn = get_db_conn();
//     let mut res = db_conn.prepare("SELECT * FROM messages").unwrap();

//         res.query([]).into_iter().for_each(|row| {
//             row.map(|x| println!("row"));
//         });

// }

fn exists(user: &str) -> bool {
    let conn = get_db_conn();
    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM users WHERE uname = ?1")
        .expect("Failed to prepare statement");

    let count: i64 = stmt.query_row(params![user], |row| row.get(0)).unwrap_or(0);

    count > 0
}

fn get_db_conn() -> rusqlite::Connection {
    rusqlite::Connection::open("database.db").expect("Failed to open database")
}
