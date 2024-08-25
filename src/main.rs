use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CacheHttp, EditMember};
use poise::serenity_prelude::FullEvent::{ReactionAdd, ReactionRemove};

pub struct Data {} 
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, _, _| {
                Box::pin(event_handler(ctx, event))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    match event {
        ReactionAdd { add_reaction } => {
            if let Ok(user) = add_reaction.user(ctx.http()).await {
                if let Some(guild) = add_reaction.guild_id {
                    let role = std::env::var("MEMBER_ROLE")?.parse::<u64>()?;
                    
                    let builder = EditMember::new()
                        .roles(vec![role]);
                    
                    guild.edit_member(ctx.http(), user.id.get(), builder).await?;
                }
            }
        }
        ReactionRemove { removed_reaction } => {
            if let Ok(user) = removed_reaction.user(ctx.http()).await {
                if let Some(guild) = removed_reaction.guild_id {
                    let builder = EditMember::new()
                        .roles(Vec::<u64>::new());
                    guild.edit_member(ctx.http(), user.id.get(), builder).await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
