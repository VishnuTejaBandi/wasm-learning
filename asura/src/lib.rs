use vt_plugin::bindings::{export, Manga, PluginError};
use vt_plugin::utils::{get_document, GetBody, VtParserUtils};
use vt_plugin::Plugin;
use waki::Client;

struct Asura;

impl Plugin for Asura {
    fn get_manga_list() -> Result<Vec<Manga>, PluginError> {
        let document = get_document(Client::new().get("https://asuracomic.net/series?page=1"))?;
        let body = document.body()?;

        let list = body.select_all("div.grid:nth-child(3) > a");
        let result = list.iter().map_while(|item| {
            let href = item.get_attr("href");
            if let Some(id) = href.split("/").nth(1) {
                Some(Manga {
                    id: "asura#".to_owned() + id,
                    image: match item.select_one("div > div > div:nth-child(1) > img") {
                        Some(el) => el.get_attr("src"),
                        None => "".into(),
                    },
                    name: match item.select_one("div > div > div:nth-child(2) > span") {
                        Some(el) => el.text().collect(),
                        None => "".into(),
                    },
                })
            } else {
                None
            }
        });

        Ok(result.collect())
    }
}

export!(Asura with_types_in vt_plugin::bindings);
