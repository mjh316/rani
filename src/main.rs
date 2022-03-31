use colorful::{Color, Colorful};
use reqwest::Client;
use scraper::{Html, Selector};
use std::io::{self, Write};
use futures::future::{join_all};
use futures::prelude::*;
use std::sync::{Arc, Mutex};
use std::process::Command;
// use std::fs::File;
mod tests; // for testing
mod decode; // for decoding url

const BASE_URL: &str = "https://gogoanime.fi";
const EPISODE_EXTENSIONS: &[&str] = &["-episode-{}", "-{}", "-episode-{}-1", "-camrip-episode-{}"];

#[allow(dead_code)]
#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut input = String::new();
    print!(
        "{}",
        String::from("Search Anime: ").color(Color::Blue).bold()
    );
    io::stdout().flush().unwrap();
    while input.len() == 0 || input.len() > 200 {
        io::stdin().read_line(&mut input)?;
    }
    input.truncate(input.trim_end().len());

    let body = client
        .get(format!("{}//search.html?keyword={}", BASE_URL, input).as_str())
        .send()
        .await?;
    let text = body.text().await?;

    let html = Html::parse_document(&text);
    let items = Selector::parse("ul.items").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let p_selector = Selector::parse("p").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut choices = Vec::new();
    let mut i: usize = 0;
    for ul in html.select(&items) {
        for li in ul.select(&li_selector) {
            for p in li.select(&p_selector) {
                for a in p.select(&a_selector) {
                    let elem = a.value();
                    let output_string = format!(
                        "{}{}{} {}",
                        "[".to_string().color(Color::Blue).bold(),
                        (i + 1).to_string()
                            .color(if i % 2 == 0 {
                                Color::Yellow
                            } else {
                                Color::Blue
                            })
                            .bold(),
                        "]".to_string().color(Color::Blue).bold(),
                        elem.attr("title")
                            .unwrap()
                            .to_string()
                            .color(if i % 2 == 0 {
                                Color::Yellow
                            } else {
                                Color::Blue
                            })
                            .bold()
                    );
                    choices.push(elem);
                    println!("{}", output_string);
                    i += 1;
                }
            }
        }
    }

    io::stdout().flush().unwrap();

    while input.len() == 0
        || !input.parse::<usize>().is_ok()
        || input.parse::<usize>().unwrap() > choices.len()
        || input.parse::<usize>().unwrap() < 1
    {
        input.clear();
        print!(
            "{}",
            String::from("Enter choice: ").color(Color::Blue).bold()
        );
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;
        input.truncate(input.trim_end().len());
        let status: bool = match input.parse::<usize>() {
            Ok(i) => {
                if i > choices.len() || i < 1 {
                    println!("{}", String::from("Invalid choice").color(Color::Red).bold());
                    false
                } else {
                    println!("{}", String::from("Loading...").color(Color::Blue).bold());
                    true
                }
            }
            Err(_) => {
                println!("{}", String::from("Invalid choice").color(Color::Red).bold());
                false
            }
        };
        if !status {
            println!("{}", "Invalid choice entered");
        }

        // println!("input: {}", input);
    }

    let choice = input.parse::<usize>().unwrap() - 1;
    let url = choices[choice].attr("href").unwrap().to_string();
    // println!("{}", url);

    let body = client
        .get(format!("{}{}", BASE_URL, url).as_str())
        .send()
        .await?;
    let text = body.text().await?;
    //let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open("anime_page.txt")?;
    //file.write_all(text.as_bytes())?;
    // println!("{}", text);
    let html = Html::parse_document(&text);
    let episode_range_selector = Selector::parse("ul#episode_page").unwrap();

    let mut min_episode = None;
    let mut max_episode = None;
    for ul in html.select(&episode_range_selector) {
        for li in ul.select(&li_selector) {
            for a in li.select(&a_selector) {
                let start: usize = a.value().attr("ep_start").unwrap().parse().unwrap();
                let end: usize = a.value().attr("ep_end").unwrap().parse().unwrap();
                if min_episode == None || start < min_episode.unwrap() {
                    min_episode = Some(start);
                }

                if max_episode == None || end > max_episode.unwrap() {
                    max_episode = Some(end);
                }
                
                // println!("{} {}", start, end);
            }
        }
    }

    let anime_id = url.split("/").last().unwrap();
    // println!("{}", anime_id);

    println!("{}{}{}", 
        "Choose episode".color(Color::Blue), 
        format!("[{}-{}]", min_episode.unwrap(), max_episode.unwrap()).color(Color::Pink3),
        ": ".color(Color::Blue)
    );

    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input)?;

    input.truncate(input.trim_end().len());

    while input.parse::<usize>().is_err() 
    || input.parse::<usize>().unwrap() < min_episode.unwrap() 
    || input.parse::<usize>().unwrap() > max_episode.unwrap() 
    {
        println!("{}", "Invalid number chosen".color(Color::Red).bold());
        input.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;
        input.truncate(input.trim_end().len());
    }

    let episode_choice = input;
    println!("Episode Choice: {}", episode_choice.clone().color(Color::Aquamarine1a));

    let mut promises = Vec::new();
    for &extension in EPISODE_EXTENSIONS {
        // println!("link: {}", format!("{}/{}{}", BASE_URL, anime_id, extension.replace("{}", &episode_choice)));
        let body = client
        .get(format!("{}/{}{}", BASE_URL, anime_id, extension.replace("{}", &episode_choice)).as_str())
        .send();
        promises.push(body);
    }
    let mut bodies = join_all(promises).then(
        |promises| 
        {
            let mut _tmp = Vec::new();
            for body in promises {
                _tmp.push(body.unwrap().text());
            }
            join_all(_tmp)
        }
    ).await;

    let url = Arc::new(Mutex::new(String::new()));
    let mut parsings = vec![];
    for result in bodies.drain(..) {
        let url = Arc::clone(&url);
        parsings.push(tokio::task::spawn_blocking(move || {
            
        let html = Html::parse_document((result).as_ref().unwrap());
        let error_selector = Selector::parse("h1.entry-title").unwrap();
        let mut bad = false;
        for error in html.select(&error_selector) {
            bad = true;
        }
        if !bad {
            let video_selector = Selector::parse("a.active").unwrap();
            for a in html.select(&video_selector) {
                // println!("{:?}", a.value());
                if a.value().attr("data-video").is_some() {
                    let mut url = url.lock().unwrap();
                    *url = a.value().attr("data-video").unwrap().to_string();
                }
            }
        }}));
    }
    for parsing in parsings {
        let _ = parsing.await?;
    }

    let url = url.lock().unwrap()[2..].to_string();
    // println!("url: {}", url);
    // YOU NEED TO PASS THE --referrer FLAG TO mpv, along with ORIGIN AND DECRYPTED URL

    // let id = decode::decode::get_id(url.as_str());
    // println!("id: {}", id);
    let output = Command::new("./decode.sh")
                    .arg(&url)
                    .output()
                    .expect("Failed to run decode script");
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.split("\n").collect::<Vec<&str>>();
    let auto_url = output[0];

    let video = Command::new("mpv")
                .arg(format!("--referrer={}", &url))
                .arg(&auto_url)
                .spawn()
                .expect("Failed to spawn mpv process");
    println!("chosen url: {}", auto_url);
    Ok(())

}
