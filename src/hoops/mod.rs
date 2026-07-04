use salvo::http::ResBody;
use salvo::prelude::*;

use crate::render_template;

pub mod auth;
pub mod cors;

#[handler]
pub async fn handle_404(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render(render_template!("error_404.html", {
            brief: if let ResBody::Error(e) = &res.body {
                e.brief.clone()
            } else {
                "Page not found".to_owned()
            }
        }));
        ctrl.skip_rest();
    }
}
