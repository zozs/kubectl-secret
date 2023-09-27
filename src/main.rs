use anyhow::Result;
use clap::Parser;
use futures::future::join_all;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
use owo_colors::OwoColorize;
use std::str;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional list of secrets to return, otherwise return everything
    names: Option<Vec<String>>,
}

async fn get_secrets(
    names: Option<Vec<String>>,
    api: Api<Secret>,
    lp: ListParams,
) -> Result<Vec<Secret>> {
    let mut result: Vec<Secret> = if let Some(n) = names {
        join_all(n.iter().map(|name| api.get(name)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
    } else {
        api.list(&lp).await.map(|l| l.items)?
    };
    result
        .iter_mut()
        .for_each(|x| x.managed_fields_mut().clear());

    Ok(result)
}

fn print_secrets(secrets: &[Secret]) -> Result<()> {
    for secret in secrets {
        // For each key in the secret, try to decode it as a UTF-8 string, and print it.
        // If it fails, we just show that the data is binary (and its length)
        if let Some(m) = &secret.data {
            let metadata = &secret.metadata;
            if let (Some(namespace), Some(name)) = (&metadata.namespace, &metadata.name) {
                println!("\n{} / {}", namespace.green(), name.blue());
            } else {
                println!("\n{}", "\n<secret with unknown namespace/name>".red());
            }
            for (k, v) in m {
                match str::from_utf8(&v.0) {
                    Ok(s) => println!("  {}: {}", k.cyan(), s),
                    Err(_) => println!(
                        "  {}: <{} {}>",
                        k,
                        "binary data of length".red(),
                        v.0.len().red()
                    ),
                };
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = Client::try_default().await?;
    let lp = ListParams::default();
    let api: Api<Secret> = Api::default_namespaced(client);
    let secrets = get_secrets(cli.names, api, lp).await?;
    print_secrets(&secrets)?;

    Ok(())
}
