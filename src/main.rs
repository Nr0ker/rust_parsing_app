use core::arch;

use reqwest;
use scraper::{Html, Selector};


// fn function_article(url: &str) -> 

fn main() -> Result<(), Box<dyn std::error::Error>> {   


    // getting the link and the status
    let response = reqwest::blocking::get("https://www.kleinanzeigen.de/s-pcs/wuerzburg/c228l7667r10").unwrap(); // GET response HTML
    let response_status = response.status();
    println!("Status {response_status}");
    
    let response_text = response.text().unwrap(); // reading the response body into a string

    let mut count: i8 = 0;
    // website block
    let document = Html::parse_document(&response_text); // parsing hrmnl as document
    let selector = Selector::parse("article h3 a").unwrap(); // making selector argument 
    for link in document.select(&selector){
        let text: String = link.text().collect();
        let href = link.value().attr("href");
        // println!("{text}");
        // println!("{href}");

        // article block


        let article_full_url: String = format!("https://www.kleinanzeigen.de{}",  href.unwrap()); // explanation here under 
        // formatting the link as the link given above is a relative one, not an absolut, so /asd/asd and not https://...

        let article_link = reqwest::blocking::get(&article_full_url).unwrap(); // GET response HTML
        let article_body_text = article_link.text().unwrap(); //  reading the response body into a string
        let article_document = Html::parse_document(&article_body_text); // parsing hrmnl as document

        
        //selecting what to selsct lul
        let article_desc_selector = Selector::parse("#viewad-description-text").unwrap();

        let article_body_desc = article_document.select(&article_desc_selector).next().unwrap(); // selecting article_selector as an argument in the HTML file
        let article_desc: String = article_body_desc.text().collect(); // getting the text


        // looking for price
        let article_price_selector = Selector::parse("h2#viewad-price").unwrap();
        let article_body_price= article_document.select(&article_price_selector).next().unwrap();
        let article_price: String = article_body_price.text().collect();

        count += 1;
        println!("{count} ----- NAME {text} ----- PRICE {article_price} ----- DESC {article_desc} ----- LINK {article_full_url}");
        
    }
    // // let body = document.select(&selector).next().unwrap(); // taking selected argument from the HTML
    // let body_value = body.value().attr("href"); // picking what is needed aka "class" or "attribute"
    // let body_text: String = body.text().collect(); // collecting the text of the article 

    // println!("{:?}", body_value);
    // println!("{:?}", body_text);

 // # -> id . -> class
    


    

    Ok(())
}
