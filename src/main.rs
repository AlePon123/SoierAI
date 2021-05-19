mod commands;
use commands::{
    ball::*,
    
};

use serenity::{
    async_trait,
    client::Client,
    framework::standard::{macros::group, StandardFramework},
    model::{channel::*, event::ResumedEvent, gateway::Ready, id::GuildId, user::User},
    prelude::{Context, EventHandler},
};



const TOKEN: &str = "DISCORD_BOT_TOKEN";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
    async fn guild_ban_addition(&self, _ctx: Context, _guild_id: GuildId, _banned_user: User) {
        println!("{} was banned", _banned_user);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!Долбаёб"
            || msg.content == "!долбаёб"
            || msg.content == "!Долбоёб"
            || msg.content == "!долбоёб"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http,format!("А я Soier , приятно познакомится <:RoflanEbalo:830110707046940682>")).await{
                println!("Error giving message: {:?}", msg);
            }
        }
        if msg.content.to_string().starts_with("+ в чат")
            || msg.content.to_string().starts_with("+ В чат")
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "+").await {
                println!("Error giving message: {:?}", msg);
            }
        }
    }
}
#[group]
#[commands(ball,)]
struct General;
#[tokio::main]
async fn main() {
    let token = &TOKEN;
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
