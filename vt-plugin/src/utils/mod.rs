use scraper::{ElementRef, Html, Selector};
use waki::RequestBuilder;

use crate::bindings::PluginError;
mod plugin_error;

pub fn get_document<'a>(req: RequestBuilder) -> Result<Html, PluginError> {
    let resp = req.send()?;
    let html = String::from_utf8(resp.body()?)?;
    Ok(Html::parse_document(&html))
}

pub trait GetBody {
    fn body(&self) -> Result<ElementRef<'_>, PluginError>;
}

impl GetBody for Html {
    fn body(&self) -> Result<ElementRef<'_>, PluginError> {
        match self.select(&Selector::parse("body").unwrap()).next() {
            Some(e) => Ok(e),
            None => Err(PluginError {
                err: "body not found".to_owned(),
            }),
        }
    }
}

pub trait VtParserUtils {
    fn get_attr(&self, attr: &str) -> String;
    fn select_one(&self, selector: &str) -> Option<ElementRef<'_>>;
    fn select_all(&self, selector: &str) -> Vec<ElementRef<'_>>;
}

impl VtParserUtils for ElementRef<'_> {
    fn get_attr(&self, attr: &str) -> String {
        match self.attr(attr) {
            Some(val) => val.into(),
            None => "".into(),
        }
    }

    fn select_one(&self, selector: &str) -> Option<ElementRef<'_>> {
        match Selector::parse(selector) {
            Ok(s) => self.select(&s).next(),
            Err(_) => None,
        }
    }

    fn select_all(&self, selector: &str) -> Vec<ElementRef<'_>> {
        match &Selector::parse(selector) {
            Ok(s) => self.select(s).collect(),
            Err(_) => vec![],
        }
    }
}
