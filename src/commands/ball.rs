use rand::*;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub async fn ball(ctx: &Context, msg: &Message) -> CommandResult {
    let result = || {
    let mut range = rand::thread_rng();
    let ball_vars = [
    "шар сказал... да",
    "шар сказал... даже не сомневайся",
    "шар сказал... еще не время",
    "шар сказал... жениться тебе пора",
    "шар сказал... забудь об этом",
    "шар сказал... звучит сомнительно",
    "шар сказал... лучше даже не жди",
    "шар сказал... лучше не надейся на это",
    "шар сказал... лучше тебе не знать",
    "шар сказал... не пиши фигню",
    "шар сказал... неплохие перспективы, дерзай",
    "шар сказал... нет",
    "шар сказал... никогда",
    "шар сказал... нужно немножно потерпеть",
    "шар сказал... однозначно!",
    "шар сказал... ответ на это не очень приятный",
    "шар сказал... пора бы тебе отдохнуть",
    "шар сказал... поступай, как велит сердце",
    "шар сказал... прислушайся к голосу разума",
    "шар сказал... раньше было лучше",
    "шар сказал... спроси еще раз",
    "шар сказал... только если удачно сложатся звезды",
    "шар сказал... тут как угадаешь",
    "шар сказал... ты шутишь?!",
    "шар сказал... уже вот-вот",
    "шар сказал... это не точно",
    "шар сказал... это слишком сложно",
    "шар сказал... это точно",
    "шар сказал... это точно"
    ];

    let random = range.gen_range(0..ball_vars.len());

    ball_vars[random]
    };
    
    msg.channel_id.say(&ctx.http, result()).await?;

    Ok(())
}

