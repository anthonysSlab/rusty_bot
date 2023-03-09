use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::Connection;
use std::fmt;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult, Args};

#[group]
#[commands(roll, join, leave, list)]
struct General;

struct Handler;

#[derive(Debug)]
pub struct Res {
    peep: String,
    country: String,
}

impl fmt::Display for Res {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {}", self.peep, self.country)
    }
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

    let token = "";
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

        let con = Connection::open("stuff.db")?;
        let mut qry = con.prepare("SELECT * FROM peeps;",)?;
        let res = qry.query_map([], |row| {
            Ok(Res {
                peep: row.get(0)?,
                country: row.get(1)?,
            })
        })?;

        let mut coun = vec![];
        let mut play = vec![];
        let mut count = 0;
    
        for re in res {
            coun.insert(count, re.as_ref().unwrap().country.to_string());
            play.insert(count, re.unwrap().peep.to_string());
            count = count + 1;
            
        }

        println!("{:?} \n{:?}", coun, play);

        // msg.reply(ctx, "Doin the Thing").await?;

        coun.shuffle(&mut thread_rng());
        println!("{}", coun);

    } else {
        msg.reply(ctx, "Permission Denied").await?;
    }

    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let con = Connection::open("stuff.db")?;
    let mut qry = con.prepare("SELECT * FROM peeps;",)?;
    let res = qry.query_map([], |row| {
        Ok(Res {
            peep: row.get(0)?,
            country: row.get(1)?,
        })
    })?;

    for re in res {
        println!("{}", re.as_ref().unwrap().to_string());
        // msg.reply(ctx, re.unwrap().to_string()).await?;
    }

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
    let con = Connection::open("stuff.db")?;
    let cnt = args.single::<String>().unwrap();
    let chk = ["Germany", "USSR", "UK", "USA", "Italy", "France", "Japan"];
    let mut chn: bool = false;

    if cnt == "help".to_string() {
        msg.reply(ctx, "Specify the amount of games. \nAlternatively: perm = permanent \nSecond argument is the country you'd like to add").await?;
    } else {
        let mut qry = con.prepare("SELECT * FROM peeps;",)?;
        let res = qry.query_map([], |row| {
            Ok(Res {
                peep: row.get(0)?,
                country: row.get(1)?,
            })
        })?;

        for re in res {
            if re.as_ref().unwrap().peep.contains(&msg.author.to_string()) {
                chn = true;
            } 
        }
    }
    
    let dat = Res {
        peep: msg.author.to_string(),
        country: (&cnt).to_string(),
    };

    if chk.contains(&cnt.as_str()) {
        println!("{}", dat.to_string());

        if chn == false {
            con.execute(
                "INSERT INTO peeps (peep, country) VALUES (?1, ?2);",
                (&dat.peep, &dat.country),
            )?;
            msg.reply(ctx, "Joined with ".to_owned() + &dat.country.to_string() + "! \n \"!leave\" to undo").await?;
        } else {
            con.execute(
                "UPDATE peeps SET country = ?1 WHERE peep = ?2;",
                (&dat.country, &dat.peep),
            )?;
            msg.reply(ctx, "Joined with ".to_owned() + &dat.country.to_string() + "! \n \"!leave\" to undo").await?;
        }
    } else {
        msg.reply(ctx, "Must Include a Valid Country! Here's a list of all: \nGermany, USSR, UK, USA, Italy, France, Japan \nMAY NOT REPEAT").await?;
    } 

    Ok(())
}