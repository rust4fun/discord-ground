use reqwest::header::CONTENT_TYPE;
use reqwest::Client as HttpClinet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Body {
    version: String,
    optimize: String,
    code: String,
    edition: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    result: String,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(prefix_command)]
pub async fn execute(ctx: Context<'_>, #[description = "rust code block"] code_block: poise::CodeBlock) -> Result<(), Error> {
    tracing::info!("execute command");
    if code_block.language.is_some_and(|x| &x == "rust") {
        let body = Body {
            version: "stable".into(),
            optimize: "0".into(),
            code: code_block.code,
            edition: "2021".into(),
        };

        ctx.say("running ...").await?;

        let a = HttpClinet::new();
        let res: Response = a
            .post("https://play.rust-lang.org/evaluate.json")
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        ctx.say(res.result).await?;
    } else {
        ctx.say("not found rust code!").await?;
    }

    Ok(())
}
