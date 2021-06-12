use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use evalexpr::eval;
#[command]
pub async fn calceval(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let evalargs = args.parse::<String>();
    let result: String; 
 
    match evalargs {
        Ok(arg) =>
            match eval(arg.as_str()) {
                Ok(res) => result = res.to_string(),
                Err(why) => {
                    msg.reply(&ctx.http, format!("Error: {}", why)).await?;
                    return Ok(()); 
                },
            }
        Err(why) => {
            msg.channel_id.say(&ctx.http, format!("Error: {}", why)).await?;
            return Ok(());
        }
    }
    
    msg.reply(&ctx.http, result).await?;

    Ok(())
    
    }

