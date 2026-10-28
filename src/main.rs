use reqwest;
use scraper::{Html, Selector};


fn main() -> Result<(), Box<dyn std::error::Error>> {   


    // getting the link and the status
    let response = reqwest::blocking::get("https://www.kleinanzeigen.de/s-pcs/wuerzburg/c228l7667r10").unwrap(); // GET response HTML
    let response_status = response.status();
    println!("Status {response_status}");
    
    let response_text = response.text().unwrap(); // reading the response body into a string


    // website block
    let document = Html::parse_document(&response_text); // parsing hrmnl as document
    let selector = Selector::parse("article h3 a").unwrap(); // making selector argument 
    let body = document.select(&selector).next().unwrap(); // taking selected argument from the HTML
    let body_value = body.value().attr("href"); // picking what is needed aka "class" or "attribute"
    let body_text: String = body.text().collect(); // collecting the text of the article 

    println!("{:?}", body_value);
    println!("{:?}", body_text);


    // article block
    let article_url: &str = body_value.unwrap(); // getting the article html picked above ^^
    let article_full_url = format!("https://www.kleinanzeigen.de{}", article_url); // explanation here under 
    // formatting the link as the link given above is a relative one, not an absolut, so /asd/asd and not https://...

    let article_link = reqwest::blocking::get(article_full_url).unwrap(); // GET response HTML
    let article_body_text = article_link.text().unwrap(); //  reading the response body into a string
    let article_document = Html::parse_document(&article_body_text); // parsing hrmnl as document

    
    //selecting what to selsct lul
    let article_selector = Selector::parse("article p").unwrap();

    let article_body = article_document.select(&article_selector).next().unwrap(); // selecting article_selector as an argument in the HTML file
    let article_text: String = article_body.text().collect(); // getting the text

    println!("NEXT {:?}", article_text);

    Ok(())
}
