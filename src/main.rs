use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult, Args};

#[group]
#[commands(roll, join, leave, list, change)]
struct General;

struct Handler;

#[derive(Debug)]
pub struct Res {
    name: String,
    country: String,
    games: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, _ready: Ready) {
        println!("Ready! \n");
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) 
        .group(&GENERAL_GROUP);

    let token = "MTA4MTkzNzI2MTE4NjI2OTI2NA.G0IGh1.QhG_nAPMudVpogO-4-fGjWrz7QBQU3TLYOHI3Y";
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("ERR: {:?}", why);
    }
}

#[command]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.to_string().eq(&"<@627015977233678336>".to_string()) {

        msg.reply(ctx, "Doin the Thing").await?;

        let mut nat = ["Germany", "USSR", "UK", "USA", "Italy", "France", "Japan"];
        let res = "";

        nat.shuffle(&mut thread_rng());
        println!("{:?}", nat);

    } else {
        msg.reply(ctx, "Permission Denied").await?;
    }

    Ok(())
}

#[command]
#[derive(Debug)]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let con = Connection::open("stuff.db")?;
    let mut qry = con.prepare("SELECT * FROM peeps;",)?;
    let res = qry.query_map(NO_PARAMS, |row| {
        Ok(Res {
            name: row.get(0)?,
            country: row.get(1)?,
            games: row.get(2)?,
        })
    })?;

    println!("{:?}", res);    

    Ok(())
}

#[command]
async fn change(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let cnt = args.single::<String>().unwrap();

    msg.reply(ctx, "Changed chosen Country to ".to_owned() + &cnt + "!").await?;

    Ok(())
}

#[command]
async fn leave(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let time = args.single::<String>().unwrap();

    if time == "".to_string() {
        msg.reply(ctx, "Specify the amount of games. \n Alternatively: perm = permanent").await?;
    } else if time == "perm".to_string() {
        msg.reply(ctx, "Left Permanently! \n \"!join\" to undo").await?;
    } else {
        let argi = time.parse::<i32>().unwrap();
        msg.reply(ctx, "Left for ".to_owned() + &argi.to_string() + " games!").await?;
    }

    Ok(())
}

#[command]
async fn join(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let time = args.single::<String>().unwrap();

    if time == "help".to_string() {
        msg.reply(ctx, "Specify the amount of games. \nAlternatively: perm = permanent \nSecond argument is the country you'd like to add").await?;
    } else {
        let cnt = args.single::<String>().unwrap();
    }
    
    if time == "perm".to_string() {
        msg.reply(ctx, "Joined Permanently! \n \"!leave\" to undo").await?;
    } else {
        let argi = time.parse::<i32>().unwrap();
        msg.reply(ctx, "Joined for ".to_owned() + &argi.to_string() + " games!").await?;
    }
    
    Ok(())
}