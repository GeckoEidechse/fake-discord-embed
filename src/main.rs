use tokio;
use warp::Reply;

use warp::{
    http::{Response, Uri},
    redirect, Filter,
};

#[tokio::main]
async fn main() {
    // Print own version number
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("Launching v{}", VERSION);

    // Logging
    // Set environt variable `RUST_LOG` to `trace` or other value to enable.
    // i.e. `export RUST_LOG=trace`
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "northstar_master_server=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let routes = warp::get()
        .and(warp::header::optional::<String>("user-agent"))
        .map(|user_agent: Option<String>| {
            dbg!(user_agent.clone());
            if let Some(user_agent) = user_agent {
                if user_agent.to_lowercase().contains("discordbot") || user_agent.to_lowercase().contains("whatsapp") || user_agent.to_lowercase().contains("mastodon") {
                    dbg!("Discordbot");
                    let body = r###"
                    <html>
                        <head>
                            <title>OnlyFans</title>
                        
                            <meta property="og:image" content="https://static.onlyfans.com/theme/onlyfans/images/og-logo.jpg?rev=202303131308-b4315db3e8">
                            <meta property="og:image:width" content="1200">
                            <meta property="og:image:height" content="628">
                            <meta property="og:description" name="description"
                                content="OnlyFans is the social platform revolutionizing creator and fan connections. The site is inclusive of artists and content creators from all genres and allows them to monetize their content while developing authentic relationships with their fanbase.">
                            <meta property="og:url" content="">
                            <meta property="og:title" content="OnlyFans">
                            <meta property="og:type" content="website">
                        </head>
                        <body>
                            <h1>Nothing to see here!</h1>
                        </body>
                    </html>"###;
                    warp::reply::html(body).into_response()
                } else {
                    dbg!("Redirecting :)");
                    redirect(Uri::from_static(
                        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
                    ))
                    .into_response()
                }
            } else {
                Response::builder()
                    .body("<html><body><h1>No user-agent header found</h1></body></html>")
                    .unwrap()
                    .into_response()
            }
        });

    println!("Starting web server");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
