#![allow(non_snake_case)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;
use SE475_L05::{Lake, Fish};


#[get("/lakes")]
fn lakes<'a>() -> content::Json<String> {
    // Temporary Stubbed Data until backend database is set up
    let sunfish: Fish = Fish {
        id: 0,
        name: String::from("Sunfish"),
    };

    let crappie: Fish = Fish {
        id: 1,
        name: String::from("Crappie"),
    };

    let pike: Fish = Fish {
        id: 2,
        name: String::from("Pike"),
    };


    let big_lake: Lake = Lake {
        id: 0,
        name: String::from("Big Lake"),
        fish: vec![sunfish.clone(), crappie.clone()],
    };

    let blue_lake: Lake = Lake {
        id: 1,
        name: String::from("Blue Lake"),
        fish: vec![pike.clone(), crappie.clone()],
    };

    let green_lake: Lake = Lake {
        id: 1,
        name: String::from("Green Lake"),
        fish: vec![sunfish.clone(), pike],
    };

    let found_lakes_complete = vec![big_lake, blue_lake, green_lake];
    let found_lakes_filtered = found_lakes_complete.iter().map(|l| (l.id, &l.name)).collect::<Vec<_>>();

    let serialized_data = serde_json::to_string(&found_lakes_filtered).unwrap();
    content::Json(serialized_data)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![lakes])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;

    #[test]
    fn test_lakes_fail_example() {
        use rocket::local::blocking::Client;

        let expected = r#"[{'id':0,'name':"Big Lake"],{'id':1,'name':"Blue Lake"},{'id':1,'name':"Green Lake"}]"#;

        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/lakes").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected.to_string()));
    }

    #[test]
    fn test_lakes_pass_example() {
        use rocket::local::blocking::Client;

        let expected = r#"[[0,"Big Lake"],[1,"Blue Lake"],[1,"Green Lake"]]"#;

        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/lakes").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected.to_string()));
    }
}
