#[cfg(test)]
mod tests {
    use actix_web::{http, test, web::Data, App};
    use libre_book::routes::init_routes;
    use libre_core::database::postgres::{init_postgres_pool, PostgresPool};
    use std::sync::Once;

    static INIT: Once = Once::new();

    async fn setup_pg() -> PostgresPool {
        INIT.call_once(|| {
            if dotenv::dotenv().is_err() {
                println!("Failed to read .env file");
            } else {
                println!(".env file loaded successfully");
            }
            std::env::set_var("RUST_LOG", "debug");
            env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
        });

        init_postgres_pool().await
    }

    #[actix_web::test]
    async fn test_list_books() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/libre/v1/books/list")
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        println!("Response: {:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_book_details() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/libre/v1/books/details/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("Response: {:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_recent_books() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/libre/v1/books/list?by=recent&&?limit=5")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("Response: {:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_top_rated_books() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/libre/v1/books/list?by=top-rated")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
