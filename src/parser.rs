use scraper::{Html, Selector};
use std::time::Duration;
use surf::{Client, Config, Url};

use crate::{
    constants::BASE_URL,
    types::{Dj, Mixtape},
};

pub struct Parser {
    client: Client,
}

impl Parser {
    pub fn new() -> Self {
        let client: Client = Config::new()
            .set_base_url(Url::parse(BASE_URL).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();
        Self { client }
    }
    pub async fn list_dj(&self) -> Result<Vec<Dj>, surf::Error> {
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(r#"div[id="block-block-22"]"#).unwrap();
        let div = document.select(&selector).next().unwrap();

        let mut results = Vec::new();

        div.select(&Selector::parse(r#"a"#).unwrap()).for_each(|a| {
            let href = a.value().attr("href").unwrap();
            let dj = a.text().collect::<Vec<_>>()[0];
            results.push(Dj {
                name: dj.to_string(),
                link: format!("{}{}", BASE_URL, href.to_string()),
            });
        });

        Ok(results)
    }

    pub async fn list_new_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".node").unwrap();

        let mut results = Vec::new();

        for element in document.select(&selector) {
            let a = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap();
            let mut href = a.value().attr("href").unwrap().to_string();
            let title = a.text().collect::<Vec<_>>()[0];
            let submitted_by = element
                .select(&Selector::parse(".submitted").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()[0]
                .to_string();
            let cover = element
                .select(&Selector::parse("img").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap()
                .to_string();
            href.remove(0);
            results.push(Mixtape {
                title: title.to_string(),
                link: format!("{}{}", BASE_URL, href.to_string()),
                cover,
                tracks: Vec::new(),
                torrent: "".to_string(),
                submitted_by,
            });
        }

        Ok(results)
    }

    pub async fn search_mixtapes(&self, query: &str) -> Result<Vec<Mixtape>, surf::Error> {
        let result = Vec::new();
        Ok(result)
    }

    pub async fn get_mixtape_details(&self, url: &str) -> Result<Mixtape, surf::Error> {
        let page = self.client.get(url).recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse("#main").unwrap();
        let content = document.select(&selector).next().unwrap();

        let title = content
            .select(&Selector::parse("h1").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0]
            .to_string();
        let cover = content
            .select(&Selector::parse("img").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap()
            .to_string();
        let tracks = content
            .select(&Selector::parse("p").unwrap())
            .collect::<Vec<_>>()[1]
            .text()
            .collect::<Vec<_>>();
        let attachment = content
            .select(&Selector::parse("#attachments").unwrap())
            .next()
            .unwrap();
        let torrent = attachment
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_string();
        let submitted_by = content
            .select(&Selector::parse(".submitted").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0]
            .to_string();

        let result = Mixtape {
            title,
            link: url.to_string(),
            cover,
            tracks: tracks
                .iter()
                .map(|t| t.replace("\n", "").to_string())
                .collect(),
            torrent,
            submitted_by,
        };
        Ok(result)
    }

    pub async fn list_popular_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse("#sidebar-right").unwrap();
        let sidebar = document.select(&selector).next().unwrap();
        let content = sidebar
            .select(&Selector::parse(".content").unwrap())
            .next()
            .unwrap()
            .select(&Selector::parse(".item-list").unwrap())
            .next()
            .unwrap();

        let mut results = Vec::new();

        for a in content.select(&Selector::parse("a").unwrap()) {
            let mut href = a.value().attr("href").unwrap().to_string();
            if a.text().collect::<Vec<_>>().is_empty() {
                continue;
            }
            let title = a.text().collect::<Vec<_>>()[0];
            href.remove(0);
            results.push(Mixtape {
                title: title.to_string(),
                link: format!("{}{}", BASE_URL, href.to_string()),
                cover: "".to_string(),
                tracks: Vec::new(),
                torrent: "".to_string(),
                submitted_by: "".to_string(),
            });
        }

        Ok(results)
    }
}
