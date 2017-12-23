extern crate aggregator;

use aggregator::{Summarizable, Tweet, NewsArticle, notify};

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!(
            "The high will be {}, and the low will be {}. The change of precipitation is {}%",
            self.high_temp, self.low_temp, self.chance_of_precipitation
        )
    }
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());

    let forecast = WeatherForecast {
        high_temp: 100.1,
        low_temp: 46.5,
        chance_of_precipitation: 12.5,
    };
    println!("1 new forecast: {}", forecast.summary());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    println!("New article available! {}", article.summary());

    notify(tweet);
    notify(forecast);
    notify(article);
}
