mod xkcd;

use std::env;

use serenity::{
    async_trait,
    client::{validate_token, Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::{
        channel::Message,
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
};

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(xkcd)]
struct Fun;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("Is the bot alive?")
                })
                .create_application_command(|command| {
                    command
                        .name("xkcd")
                        .description("Returns a comic from xkcd")
                        .create_option(|option| {
                            option
                                .name("number")
                                .description("The number of the comic")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(false)
                        })
                })
        })
        .await;

        println!("Added the following global slash commands: {:#?}", commands);
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Pong!".to_string(),
                "xkcd" => {
                    let options = command.data.options.get(0);

                    let num = options
                        .and_then(|o| o.resolved.as_ref())
                        .and_then(|v| {
                            if let ApplicationCommandInteractionDataOptionValue::Integer(n) = v {
                                Some(n)
                            } else {
                                None
                            }
                        })
                        .map(|i| *i as u32);

                    let comic = match num {
                        Some(num) => xkcd::Comic::from_num(num),
                        None => xkcd::Comic::current(),
                    };

                    match comic {
                        Some(comic) => {
                            let embed = command
                                .create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(InteractionResponseType::ChannelMessageWithSource)
                                        .interaction_response_data(|message| {
                                            message.create_embed(|e| {
                                                e.image(comic.img_url.to_string());
                                                e.title(format!("xkcd n°{}", comic.num));
                                                e.url(comic.link);
                                                e.field("Title", comic.title.to_string(), false);
                                                e.field("Alt", comic.alt.to_string(), false);
                                                e.footer(|f| {
                                                    f.text(format!(
                                                        "From {}",
                                                        comic.date.format("%d/%m/%Y")
                                                    ));
                                                    f
                                                });
                                                e
                                            })
                                        })
                                })
                                .await;

                            if let Err(why) = embed {
                                eprintln!("Error sending slash command embed response {:?}", why);
                            }
                            return;
                        }
                        None => "Comic not found".to_string(),
                    }
                }
                _ => "Not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

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

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Missing application id in the environment")
        .parse()
        .expect("The application id is not a valid id");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(application_id)
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
            // msg.reply(ctx, comic.img_url).await?;
            let embed = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.image(comic.img_url.to_string());
                        e.title(format!("xkcd n°{}", comic.num));
                        e.url(comic.link);
                        e.field("Title", comic.title.to_string(), true);
                        e.field("Alt", comic.alt.to_string(), true);
                        // e.field("Date", comic.date.format("%d/%m/%Y"), false);
                        // e.timestamp(comic.date);
                        e.footer(|f| {
                            f.text(format!("From {}", comic.date.format("%d/%m/%Y")));
                            f
                        });
                        e
                    })
                })
                .await;

            if let Err(why) = embed {
                eprintln!("Error sending message {:?}", why);
            }
        }
        None => {
            msg.reply(ctx, "Comic not found.").await?;
        }
    }

    Ok(())
}
