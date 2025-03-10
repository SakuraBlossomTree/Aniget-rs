use std::io::{self, BufReader};
use std::fs::File;
use scraper::{Html, Selector};
use std::process::Command;

fn main(){
    let base_url = "https://nyaa.si";
    let search_url = "?q=";
    let seeders = "&s=seeders";
    let download_query = "/download/";
    let mut links: Vec<String> = Vec::new();
    clearscreen::clear().expect("failed to clear screen");
    println!("Enter your anime name");
    let mut input = String::new();
    let mut choice = String::new(); 
    let file_path = "downloaded.torrent";
    

    io::stdin().read_line(&mut input).expect("failed to readline");
    println!("Your input {}", input);
    let cleaned_input = input.trim().to_lowercase().replace(" ", "+");
    let url = format!("{}{}{}{}", base_url, search_url, cleaned_input, seeders);
    println!("{}", url);
    
    let body = ureq::get(&url)
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let document = Html::parse_document(&body);

    let selector = Selector::parse("a").unwrap();
    let mut i = 0;

    for title in document.select(&selector){
        if let Some(title) = title.value().attr("title"){
            let word_count = title.split_whitespace().count();
            if word_count > 3{
                i+=1;
                println!("{} {}", i,title);
            }
        }
    }


    for element in document.select(&selector){
        if let Some(title) = element.value().attr("title"){
            let title_cleaned = title.trim().to_lowercase();
            let word_count = title_cleaned.split_whitespace().count();
            if word_count > 3{
                if let Some(link) = element.value().attr("href"){
                    let clean_link = link.replace("/view/", ""); 
                    links.push(clean_link);
                }
            }
        }
    }

    println!("{:?}", links);
    println!("{}", links.len());
   
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let number: usize = choice.trim().parse().expect("Invalid number");

    if number <= links.len(){
        let selected_link = &links[number - 1];
        let selected_link = format!("{}{}", selected_link, ".torrent");
        println!("{}" , selected_link);
        let torrent = format!("{}{}{}", base_url, download_query,selected_link);
        println!("{}", torrent);
        let response = ureq::get(&torrent)
            .call()
            .unwrap()
            .into_reader();
        let mut reader = BufReader::new(response);
        let mut file = File::create(file_path).unwrap();
        std::io::copy(&mut reader, &mut file).unwrap();

        println!("{}", file_path);
    } else {
        println!("Please enter the right choice");
    }

    let _ =Command::new("aria2c")
        .arg("-x8")
        .arg("downloaded.torrent")
        .spawn()
        .expect("Failed to execute process")
        .wait();
}
