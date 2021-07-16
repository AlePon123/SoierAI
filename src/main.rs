#[warn(non_snake_case)]

mod commands;
use commands::{
    ball::*,
    eval::*,
    addcommand::*,
};
use serenity::{
    async_trait,
    client::Client,
    framework::standard::{macros::group, StandardFramework},
    model::{
        channel::*, event::ResumedEvent, gateway::Ready, guild::Member, id::GuildId, user::User,
    },
    prelude::{Context, EventHandler},
};
const TOKEN: &str = "ODI5Nzg0NDgxNTA1MzQ1NjI5.YG9LBQ.7tlexE3sXbsaFg7DAspNQjyO_Z8";

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

    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, mut new_member: Member) {
        println!("{} join on server", new_member);
        if let Err(why) = new_member.add_role(&ctx.http, 847210252168724489).await {
            println!("Error giving role to new member in guild {}", why);
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!Долбаёб"|| msg.content == "!долбаёб"|| msg.content == "!Долбоёб"|| msg.content == "!долбоёб" {
            if let Err(_) = msg.reply(&ctx.http,format!("А я Soier , приятно познакомится <:RoflanEbalo:830110707046940682>")).await {
                println!("Error giving message: {:?}", msg);
            }
        }
        if msg.content.to_string().starts_with("+ в чат")|| msg.content.to_string().starts_with("+ В чат") {
            if let Err(_) = msg.channel_id.say(&ctx.http, "+").await {
                println!("Error giving message: {:?}", msg);
            }
        }
    }
}
#[group]
#[commands(ball,eval,addtextcommand,testq)]
struct General;

#[tokio::main]
async fn main() {
    let token = &TOKEN;
    let framework = StandardFramework::new()
        .configure(|c|c.prefix("!"))
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
