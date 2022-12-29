use std::{io::Cursor, sync::Arc};

async fn login() {
    // NOTE: CookieStore の実装として reqwest::cookie::Jar もあるが、
    //       これは状態をファイル等に保存することができず、一時的な利用にしか使えない (1敗)
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::default();
    let cookie_store = Arc::new(cookie_store);
    let client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap();
    let login_url = "https://example.com/login";
    let form = [("username", "****"), ("password", "****")];
    let res = client.post(login_url).form(&form).send().await.unwrap();
    let text = res.text().await.unwrap();
    println!(
        "{} {}",
        text.contains("ログイン"),
        text.contains("ログアウト")
    );

    let mypage_url = "https://example.com/mypage";
    let res = client.get(mypage_url).send().await.unwrap();
    let text = res.text().await.unwrap();
    println!(
        "{} {}",
        text.contains("ログイン"),
        text.contains("ログアウト")
    );

    {
        let mut buf: Vec<u8> = Vec::new();
        let store = cookie_store.lock().unwrap();
        // NOTE: save_json() メソッドでは { "expires": "SessionEnd" } 等の cookie を
        //       保存することができない (1敗)
        store
            .save_incl_expired_and_nonpersistent_json(&mut buf)
            .unwrap();
        println!("{}", String::from_utf8_lossy(&buf));
    }
}

async fn request_with_cookie() {
    let cookie = r#"{"raw_cookie":"SESSIONID=****; Secure; Path=/","path":["/",true],"domain":{"HostOnly":"exmalep.com"},"expires":"SessionEnd"}
"#;
    let cursor = Cursor::new(String::from(cookie).into_bytes());
    let cookie_store = reqwest_cookie_store::CookieStore::load_json(cursor).unwrap();
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);
    let client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap();
    let mypage_url = "https://example.com/mypage";
    let res = client.get(mypage_url).send().await.unwrap();
    let text = res.text().await.unwrap();
    println!(
        "{} {}",
        text.contains("ログイン"),
        text.contains("ログアウト")
    );
}

#[tokio::main]
async fn main() {
    // login().await;
    request_with_cookie().await;
}
