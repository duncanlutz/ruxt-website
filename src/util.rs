use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
struct DocPageLink {
    title: String,
    route: String,
}

#[derive(Serialize, Debug)]
struct DocPage {
    name: String,
    links: Vec<DocPageLink>,
}

fn get_doc_page_links() -> Vec<DocPageLink> {
    vec![
        DocPageLink {
            title: "Getting Started".to_string(),
            route: "/docs".to_string(),
        },
        DocPageLink {
            title: "Installation".to_string(),
            route: "/docs/installation".to_string(),
        },
        DocPageLink {
            title: "Commands".to_string(),
            route: "/docs/commands".to_string(),
        },
        DocPageLink {
            title: "Routing".to_string(),
            route: "/docs/routing".to_string(),
        },
        DocPageLink {
            title: "Dynamic Paths".to_string(),
            route: "/docs/dynamic_paths".to_string(),
        },
    ]
}

pub fn render_doc(
    tera: &tera::Tera,
    page_name: &str,
    template_loc: &str,
) -> Result<HttpResponse, actix_web::Error> {
    let mut ctx = tera::Context::new();
    let links = get_doc_page_links();
    let current_page_index = links.iter().position(|link| link.title == page_name);

    match current_page_index {
        None => Err(actix_web::error::ErrorNotFound("Page not found")),
        Some(ind) => {
            let links_for_iter = links.clone();
            let next_page: Option<&DocPageLink> = links_for_iter.get(ind + 1);

            let prev_page: Option<&DocPageLink> = if ind == 0 {
                None
            } else {
                links_for_iter.get(ind - 1)
            };

            let page = DocPage {
                name: page_name.to_string(),
                links,
            };

            ctx.insert("next_page", &next_page);
            ctx.insert("prev_page", &prev_page);

            ctx.insert("page", &page);

            let rendered = tera.render(template_loc, &ctx).map_err(|e| {
                eprintln!("Error rendering template: {}", e);
                actix_web::error::ErrorInternalServerError("Error rendering template")
            })?;

            Ok(HttpResponse::Ok().body(rendered))
        }
    }
}
