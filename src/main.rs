use std::io::{ stdout, Write };

use curl::easy::{
    Easy,
    List
};

struct Param {
    name: String,
    value: String
}

struct Url {
    url: String,
    params: Vec<Param>
}

impl Url {
    fn new(url: String) -> Url {
        Url {
            url,
            params: Vec::new()
        }
    }

    fn add_param(mut self, param: Param) -> Self {
        self.params.push(param);
        self
    }

    fn to_string(&mut self) -> String {
        let mut delimiter = "?";
        let mut url = &mut self.url;
        for param in self.params.iter() {
            let mut concat = String::from(delimiter);
            concat.push_str(&param.name);
            concat.push_str("=");
            concat.push_str(&param.value);

            url.push_str(&concat);
            delimiter = "&";
        }
        url.to_string()
    }
}

fn main() {
    let mut easy = Easy::new();

    let url = Url::new("https://the-cocktail-db.p.rapidapi.com/filter.php".to_string())
        .add_param(Param {
            name: String::from("i"),
            value: String::from("Gin"),
        })
        .to_string();

    let mut header_list = List::new();
    header_list.append("X-RapidAPI-Key: 20cf58e750msh29cd2f8d998cf44p1d5943jsne20d8e3e2c32").unwrap();
    header_list.append("X-RapidAPI-Host: the-cocktail-db.p.rapidapi.com").unwrap();

    easy.url(&url).unwrap();
    if let Ok(()) = easy.http_headers(header_list) {
        println!("We have added headers!");
    }

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}
