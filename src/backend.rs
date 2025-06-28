use dioxus::prelude::*;
//mod components;
//use crate::components::*;



#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError>{
    DB.with(|f| f.execute("INSERT Into dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER by id DESC limit 10")
        .unwrap()
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .map(|r| r.unwrap())
            .collect()
    });
    Ok(dogs)
}


#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open");
        conn.execute_batch(
            "CREATE table if not exists dogs (
                id INTEGER primary key,
                url TEXT NOT NULL
            );",

        ).unwrap();
        conn
    }
}


