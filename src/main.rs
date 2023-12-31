use actix_web::*;

#[cfg(feature = "ssr")]
#[get("/styles.css")]
async fn css() -> impl Responder {
    actix_files::NamedFile::open_async("target/site/styles/styles.css").await
}

#[cfg(feature = "ssr")]
#[get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use website::app::*;
    use sqlx::{Pool, Sqlite};

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });
    //TODO change pool options
    let pool: Pool<Sqlite> = Pool::connect_lazy("sqlite:/home/cdimegl1/LeagueAutoBet/bets.db").unwrap();

    // Explicit server function registration is no longer required
    // on the main branch. On 0.3.0 and earlier, uncomment the lines
    // below to register the server functions.
    // _ = GetPost::register();
    // _ = ListPostMetadata::register();
    //GetBets::register_explicit();

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(css)
            .service(favicon)
            //.service(Files::new("/assets", site_root))
            //.service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), |cx| view! { cx, <App/> })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(web::Data::new(pool.clone()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

