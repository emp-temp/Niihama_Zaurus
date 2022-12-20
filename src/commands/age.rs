use poise::serenity_prelude as serenity;

use crate::common::types;

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: types::Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), types::Error> {
    let u = user.as_ref().unwrap();
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}