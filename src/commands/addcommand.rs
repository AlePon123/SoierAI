use std::fmt;
use serenity::{framework::standard::{
   Args, CommandResult,
    macros::command,
}, model::prelude::*, prelude::*};

static mut COMMAND_LIST:Vec<Command> = Vec::new();

#[derive(Default, Clone,)]
pub struct Command {
    pub title: String,
    pub value: String,
}

impl Command {
    fn new(title: String, value: String) -> Command {
        Command { title, value } 
    }
}

impl fmt::Display for Command { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} # {}", self.title, self.value)
    }
}

#[command]
pub async fn addtextcommand(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let newtitle = args.single::<String>()?;
    unsafe {
        for i in &COMMAND_LIST{
            if i.title == newtitle {
                msg.channel_id.say(ctx,"Такой заголовок уже существует").await?;
                return Ok(());
            } 
        }
    } 

    let value = args.single::<String>()?;
    let cmd = Command::new(newtitle, value);
    msg.channel_id.say(ctx,format!("Command has been added {}",&cmd)).await?;
    unsafe {
        COMMAND_LIST.push(cmd);
    }
    Ok(())
}
#[command] 
pub async fn testq(ctx:&Context,msg:&Message) -> CommandResult {
    unsafe {  
        if COMMAND_LIST.len() == 0 {
            msg.channel_id.say(ctx,"Список команд пуст").await?;
            return Ok(());
        }

        for cmd in &COMMAND_LIST {
            msg.channel_id.say(ctx,format!("Command {}", &cmd)).await?;
        }
    }
    Ok(())
}
