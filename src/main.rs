use bisky::atproto::ClientBuilder;
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::Post;

#[tokio::main]
async fn main() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    let text = std::env::args().collect::<Vec<String>>()[1..].join(" ");

    let mut vec = vec![text];

    if grep_cli::is_readable_stdin() {
        let mut pipe = String::new();

        std::io::Read::read_to_string(&mut std::io::stdin(), &mut pipe).unwrap();

        vec.push(pipe);
    }

    let text = vec.join("\n\n").trim();

    let mut client = ClientBuilder::default().build().unwrap();

    client
        .login(
            &url::Url::parse(
                &std::env::var("SKEET_HOST").unwrap_or("https://bsky.social".to_string()),
            )
            .unwrap(),
            &std::env::var("SKEET_USERNAME").expect("Set SKEET_USERNAME as environment variable"),
            &std::env::var("SKEET_PASSWORD").expect("Set SKEET_PASSWORD as environment variable"),
        )
        .await
        .unwrap();

    let mut bsky = Bluesky::new(client);

    bsky.me()
        .unwrap()
        .post(Post {
            text,
            created_at: chrono::Utc::now(),
            rust_type: None,
            embed: None,
            reply: None,
        })
        .await
        .unwrap();
}
