use salvo::prelude::*;

use crate::{AppResult, render_template};

#[handler]
pub async fn hello(req: &mut Request) -> AppResult<Text<String>> {
    Ok(render_template!(
        "hello.html", {
            name: req.query::<&str>("name").unwrap_or("World")
        }
    ))
}
