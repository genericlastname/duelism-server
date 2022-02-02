use reqwest;

#[tokio::main]
async fn main() {
    let result =
        reqwest::get("https://db.ygoprodeck.com/api/v7/cardinfo.php?name=Tornado%20Dragon").await;
    println!("{:?}", result);
}
