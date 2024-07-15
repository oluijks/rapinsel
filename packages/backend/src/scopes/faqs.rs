use actix_web::web;

use crate::api::faqs::{
    create_faq_handler, delete_faq_handler, fetch_faq_handler, fetch_faqs_handler,
    update_faq_handler,
};

pub fn scoped_faqs_config(cfg: &mut web::ServiceConfig) {
    cfg.service(fetch_faq_handler)
        .service(fetch_faqs_handler)
        .service(create_faq_handler)
        .service(update_faq_handler)
        .service(delete_faq_handler);
}
