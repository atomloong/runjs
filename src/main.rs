use deno_core::{anyhow::Result, JsRuntime, RuntimeOptions};

#[tokio::main]
async fn main()->Result<()>{
    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    let code = r#"
        Deno.core.print("Hello World");
    "#;
    runtime.execute_script("<anon>",code)?;
    Ok(())
}
