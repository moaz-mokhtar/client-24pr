use client_24pr::USERS_SAMPLE;
use error_chain::error_chain;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct Organisation {
    login: String,
    avatar_url: String,
    link: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PullRequest {
    title: String,
    issue_url: String,
    repo_name: String,
    body: String,
    created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u32,
    nickname: String,
    gravatar_id: String,
    github_profile: String,
    contributions_count: u32,
    link: String,
    organisations: Vec<Organisation>,
    pull_requests: Vec<PullRequest>,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Json(serde_json::Error);
    }
}

fn main() -> Result<()> {
    // let mut res = reqwest::blocking::get("https://24pullrequests.com/users.json?page=1")?;
    // let mut body = String::new();
    // res.read_to_string(&mut body)?;

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    // let data = r#"
    //     {
    //         "id": 183,
    //         "nickname": "andrew",
    //         "gravatar_id": "8ddbf811da78bb0daeeb3cacd7cf743f",
    //         "github_profile": "https://github.com/andrew",
    //         "contributions_count": 15,
    //         "link": "http://localhost:3000/users/andrew",
    //         "organisations": [
    //             {
    //                 "login": "eius",
    //                 "avatar_url": "https://1.gravatar.com/avatar/.....",
    //                 "link": "http://localhost:3000/organisations/5"
    //             },
    //             {
    //                 "login": "corrupti",
    //                 "avatar_url": "https://1.gravatar.com/avatar/.....",
    //                 "link": "http://localhost:3000/organisations/66"
    //             },
    //             {
    //                 "login": "soluta",
    //                 "avatar_url": "https://1.gravatar.com/avatar/.....",
    //                 "link": "http://localhost:3000/organisations/86"
    //             }
    //         ],
    //         "pull_requests": [
    //             {
    //                 "title": "molestiae",
    //                 "issue_url": "http://braunhilpert.org/hardy",
    //                 "repo_name": "fugiat",
    //                 "body": "Velit dicta ratione maxime rerum qui aut neque.",
    //                 "created_at": "2013-11-25T17:47:32.249Z"
    //             },
    //             {
    //                 "title": "eveniet",
    //                 "issue_url": "http://jacobshagenes.name/alba_hintz",
    //                 "repo_name": "maiores",
    //                 "body": "Assumenda quo veritatis non vero fugiat voluptatem rerum.",
    //                 "created_at": "2013-11-26T17:47:32.244Z"
    //             },
    //             {
    //                 "title": "et",
    //                 "issue_url": "http://hane.info/dameon",
    //                 "repo_name": "ipsam",
    //                 "body": "Omnis temporibus quia nobis quia labore officiis.",
    //                 "created_at": "2013-11-27T17:47:32.239Z"
    //             },
    //             {
    //                 "title": "eos",
    //                 "issue_url": "http://glover.info/zella_erdman",
    //                 "repo_name": "tenetur",
    //                 "body": "Quibusdam quia autem ipsam. Maiores dolor dolorem rerum sunt.",
    //                 "created_at": "2013-11-28T17:47:32.234Z"
    //             },
    //             {
    //                 "title": "voluptatum",
    //                 "issue_url": "http://gutkowski.info/cecil",
    //                 "repo_name": "eos",
    //                 "body": "Atque qui minus officiis facere. Nam consequatur consequuntur.",
    //                 "created_at": "2013-12-09T17:47:32.170Z"
    //             }
    //         ]
    //     }"#;

    let users: Vec<User> = serde_json::from_str(USERS_SAMPLE).unwrap();

    // let url = "https://24pullrequests.com/users.json?page=1";
    // let users_json = reqwest::blocking::get(url)?.text()?;
    // let users: Vec<User> = serde_json::from_str(&users_json)?;

    users.iter().for_each(|user| {
        println!("===========");
        println!("{:#?}", user);
    });

    Ok(())
}
