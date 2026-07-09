use salvo::prelude::*;

use crate::render_template;

#[handler]
pub async fn homepage_page(req: &mut Request, res: &mut Response) {
    render_template!(res,
        "hello.html", {
            name: req.query::<&str>("name").unwrap_or("World")
        }
    );
}
