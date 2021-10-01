mod xkcd;

use serenity::async_trait;
use serenity::client::{validate_token, Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

use std::env;

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(xkcd)]
struct Fun;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP)
        .group(&FUN_GROUP);

    // Load environment variables from ./.env
    dotenv::dotenv().expect("Could not load .env file");
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    if validate_token(token.clone()).is_err() {
        panic!("Invalid format for bot token: {}", token)
    }

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn xkcd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let comic = match args.single::<u32>() {
        Ok(num) => xkcd::Comic::from_num(num),
        Err(_) => xkcd::Comic::current(),
    };
    match comic {
        Some(comic) => {
            msg.reply(ctx, comic.img_url).await?;
        }
        None => {
            msg.reply(ctx, "Comic not found.").await?;
        }
    }

    Ok(())
}
