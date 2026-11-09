use serde::{Serialize, Deserialize};
use reqwest;
use scraper::{Html, Selector};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct JsonStructure {
    title: String,
    price: String,
    description: String,
    link: String
}


fn main() -> Result<(), Box<dyn std::error::Error>> {   

// https://www.kleinanzeigen.de/s-pcs/wuerzburg/seite:2/c228l7667r10
// https://www.kleinanzeigen.de/s-pcs/wuerzburg/c228l7667r10
   
//calendarOutline
    

    let mut to_json: Vec<JsonStructure> = vec![]; // making a to_json variable so later on i can write the response from the next code to json

    // reading json file
    // let json_content = fs::read_to_string("data.json").unwrap();
    // let json_read_content: Vec<JsonStructure> = serde_json::from_str(&json_content).unwrap(); 
    // getting the link and the status
    for i in 1..6{

        
        let url = format!("https://www.kleinanzeigen.de/s-pcs/wuerzburg/seite:{}/c228l7667r10", i);
        let response = reqwest::blocking::get(&url).unwrap(); // GET response HTML
        let response_status = response.status();
        println!("Status {response_status}");
        
        let response_text = response.text().unwrap(); // reading the response body into a string

        
        // website block
        let document = Html::parse_document(&response_text); // parsing hrmnl as document
        let selector = Selector::parse("article h3 a").unwrap(); // making selector argument 


        for link in document.select(&selector){

            

            let text: String = link.text().collect();
            let href = link.value().attr("href");


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

            // // looking for upload date
            // let select_article_upload_date = Selector::parse( "#viewad-main-info > div.flex.flex-col.gap-xxsmall > div:nth-child(2) > div > span").unwrap();
            // let parse_article_upload_date = article_document.select(&select_article_upload_date)
            // .next()
            // .map(|el| el.text().collect())
            // .unwrap_or_else(|| "N/A".to_string());
            // // let article_upload_date: String = parse_article_upload_date.text().collect();
            // print!("{parse_article_upload_date}");

            to_json.push(JsonStructure{title: text, price: article_price, description: article_desc, link: article_full_url}) // write data to to_json variable
                
          

            
        }
    }

    std::fs::write("raw_data.json", serde_json::to_string_pretty(&to_json).unwrap()).unwrap(); // write to data.json file
  
 // # -> id . -> class
    


    

    Ok(())
}
