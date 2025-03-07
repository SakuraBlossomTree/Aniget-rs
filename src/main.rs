use std::io::{self, BufReader};
use std::fs::File;
use scraper::{Html, Selector};

fn main(){
    let base_url = "https://nyaa.si";
    let search_url = "?q=";
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
    let url = format!("{}{}{}", base_url, search_url, cleaned_input);
    println!("{}", url);
    
    let body = ureq::get(&url)
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let document = Html::parse_document(&body);

    let selector = Selector::parse("a").unwrap();

    for title in document.select(&selector){
        println!("{}", title.value().attr("title").into_iter().collect::<String>());
    }


    for element in document.select(&selector){
        if let Some(title) = element.value().attr("title"){
            let title_cleaned = title.trim().to_lowercase();
            let word_count = title_cleaned.split_whitespace().count();
            if word_count > 3{
                if let Some(link) = element.value().attr("href"){
                    // println!("{}", link);
                    let clean_link = link.replace("/view/", ""); 
                    links.push(clean_link);
                }
            }
        }
    }
    println!("{:?}", links);
   
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let number: usize = choice.trim().parse().expect("Invalid number");
    // println!("You entered: {}", number);

    if number <= links.len(){
        let selected_link = &links[number];
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

}
