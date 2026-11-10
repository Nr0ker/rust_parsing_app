use serde::{Serialize, Deserialize};
use reqwest;
use scraper::{Html, Selector};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct RawData {
    title: String,
    price: String,
    description: String,
    link: String
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct GetArticleId {
    link: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {   

// https://www.kleinanzeigen.de/s-pcs/wuerzburg/seite:2/c228l7667r10
// https://www.kleinanzeigen.de/s-pcs/wuerzburg/c228l7667r10
   
//calendarOutline
    

    let mut to_json: Vec<RawData> = vec![]; // making a to_json variable so later on i can write the response from the next code to json
    let mut on_repeat: i8 = 0;
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

            let existing_article_id: Vec<GetArticleId> = serde_json::from_str(&std::fs::read_to_string("raw_data.json")?)?;   

            if existing_article_id.iter().any(|x|x.link == article_full_url){
                on_repeat += 1; // on_repeat is here to check if existing_article_id == article_full_url 3 times, so if found it doesnt do anything anymore
                if on_repeat == 3{
                     break;
                }
            }else{
                to_json.push(RawData{title: text, price: article_price, description: article_desc, link: article_full_url}) // write data to to_json variable     
            }

                
          

            
        }
    }

    std::fs::write("raw_data.json", serde_json::to_string_pretty(&to_json).unwrap()).unwrap(); // write to data.json file

  
 // # -> id . -> class
    


    

    Ok(())
}
