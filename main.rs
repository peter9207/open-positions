use std::error::Error;
use scraper::{Html, Selector};
use std::collections::LinkedList;

struct Link {
    depth: i32,
    url: String,
}

async fn fetch_html(url: String) -> Result<String, Box<dyn Error>>{
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(resp)
}

async fn craw(input: String)  -> Result<&'static str, Box<dyn Error>>{
    let mut queue = LinkedList::new();
    queue.push_front(Link{
        depth: 0,
        url: input, 
    });


    while !queue.is_empty(){

        let first = queue.pop_front().ok_or("empty first value")?;
        let html = fetch_html(first.url).await?;
        let dom = Html::parse_document(html.as_str());

        let selector = Selector::parse("a").unwrap();
        for element in dom.select(&selector){

            let next_link = element.value().attr("href").ok_or("")?;

            if first.depth <= 1 && next_link != "" && next_link.starts_with("http"){
                println!("added link {} {} ", first.depth, element.value().attr("href").ok_or("")?);
                queue.push_back(Link{
                    depth:first.depth + 1,
                    url: next_link.to_owned(),
                })
            }
        }

    }
    Ok("ok")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = String::from("https://news.ycombinator.com/item?id=37351667");
    // let resp = fetch_html(url).await?;
    craw(url).await?;

    Ok(())
}
