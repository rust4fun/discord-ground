type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(prefix_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    tracing::info!("hello command");
    ctx.say("Hello!").await?;

    Ok(())
}
