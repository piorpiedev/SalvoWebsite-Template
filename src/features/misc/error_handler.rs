use salvo::prelude::*;

#[handler]
pub async fn error_page(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    // if let Some(StatusCode::NOT_FOUND) = res.status_code {
    //     render_template!(res, "error_404.html", {
    //         brief:
    //             if let ResBody::Error(e) = &res.body {
    //                 e.brief.clone()
    //             } else {
    //                 "Page not found".to_owned()
    //             }
    //     });
    //     ctrl.skip_rest();
    // }
}
