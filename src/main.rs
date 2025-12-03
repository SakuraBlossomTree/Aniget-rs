use std::io::{self, BufReader};
use std::fs::File;
use scraper::{Html, Selector};
use std::process::Command;


fn get_torrent_files() -> Vec<(usize, String)> {
    println!("--- DEBUG: Running aria2c --show-files ---");

    let output = Command::new("aria2c")
        .arg("--show-files")
        .arg("downloaded.torrent")
        .output()
        .expect("Failed to run aria2c --show-files");

    println!("--- DEBUG: aria2c exited with status: {:?}", output.status);

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- DEBUG: Raw aria2c output start ---");
    println!("{}", stdout);
    println!("--- DEBUG: Raw aria2c output end ---");

    let mut files = Vec::new();

    println!("--- DEBUG: Starting line-by-line parse ---");

    for (line_number, line) in stdout.lines().enumerate() {
        println!("DEBUG Line {}: '{}'", line_number, line);

        let line = line.trim();
        if line.is_empty() {
            println!("  -> Skipped (empty)");
            continue;
        }

        // Debug split
        let parts: Vec<&str> = line.split('|').collect();
        println!("  -> Split parts: {:?}", parts);

        if parts.len() == 2 {
            let idx_str = parts[0].trim();
            let path_str = parts[1].trim();
            println!("  -> idx_str='{}', path_str='{}'", idx_str, path_str);

            if let Ok(idx) = idx_str.parse::<usize>() {
                println!("  -> Parsed idx OK = {}", idx);

                // Debug the filtering logic
                let matches_path = path_str.starts_with("./")
                    || path_str.ends_with(".mkv")
                    || path_str.ends_with(".mp4")
                    || path_str.ends_with(".avi");

                println!("  -> Path matches filtering rules? {}", matches_path);

                if matches_path {
                    println!("  -> ADDED FILE ({}, '{}')", idx, path_str);
                    files.push((idx, path_str.to_string()));
                } else {
                    println!("  -> Rejected by filtering");
                }
            } else {
                println!("  -> idx_str failed to parse as usize");
            }
        } else {
            println!("  -> Not a valid aria2c file line");
        }
    }

    println!("--- DEBUG: Final file list = {:?} ---", files);

    files
}

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

    let files = get_torrent_files();

    if files.is_empty() {
        println!("No files found in torrent!");
        return;
    }

    println!("\nFiles inside torrent:");
    for (i, (idx, name)) in files.iter().enumerate() {
        println!("{}: {}", idx, name);
    }

    println!("Enter episode/file index to download:");
    let mut ep_choice = String::new();
    io::stdin().read_line(&mut ep_choice).unwrap();
    let ep_idx: usize = ep_choice.trim().parse().unwrap_or(0);

    if ep_idx == 0 || ep_idx > files.len() {
        println!("Invalid selection!");
        return;
    }

    println!("Downloading file index {}...", ep_idx);

    Command::new("aria2c")
        .arg("--select-file")
        .arg(format!("{}", ep_idx))
        .arg("downloaded.torrent")
        .spawn()
        .expect("Failed running aria2c")
        .wait()
        .unwrap();
    
        
}
