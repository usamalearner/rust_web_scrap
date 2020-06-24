#![allow(non_snake_case)]
use reqwest;  
use scraper::{Html, Selector};     
fn main() -> Result<(), Box<dyn std::error::Error>> {
      let mut url = reqwest::get("https://www.neduet.edu.pk/")?;
      assert!(url.status().is_success());

      let news=url.text().unwrap();
      let parse=Html::parse_document(&news);
      let sel=Selector::parse(".heading-inner>h2 ,.datetimecss , .news-events > div > p > a
").unwrap();

println!("\n Latest News in NED \n ==================");


      for latest in parse.select(&sel){
        let full_news=latest.text().collect::<Vec<_>>();
        if full_news.len() == 1 {
            println!("{:?} ",full_news[0]);

        }
    }
        Ok(())
        
       
    }
