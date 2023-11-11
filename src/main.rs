use std::{error::Error, time::Duration};

use clap::{Parser, Subcommand};
use zbus::Connection;

#[derive(Debug, Parser)]
#[clap(name = "pomc", version)]
struct Pomc {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Start,
    Pause,
    Stop,
    Skip,
    GetIteration,
    GetRemaining,
    IsRunning,
    IsOnBreak,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Pomc::parse();
    let connection = Connection::session().await?;

    match args.command {
        Command::Start => start(&connection).await?,
        Command::Pause => pause(&connection).await?,
        Command::Stop => stop(&connection).await?,
        Command::Skip => skip(&connection).await?,
        Command::GetIteration => get_iteration(&connection).await?,
        Command::GetRemaining => get_remaining(&connection).await?,
        Command::IsRunning => is_running(&connection).await?,
        Command::IsOnBreak => is_on_break(&connection).await?,
    }

    Ok(())
}

async fn start(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "Start",
            &(),
        )
        .await?;
    Ok(())
}

async fn pause(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "Pause",
            &(),
        )
        .await?;
    Ok(())
}

async fn stop(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "Stop",
            &(),
        )
        .await?;
    Ok(())
}

async fn skip(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "Skip",
            &(),
        )
        .await?;
    Ok(())
}

async fn get_iteration(connection: &Connection) -> Result<(), Box<dyn Error>> {
    let m = connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "GetIteration",
            &(),
        )
        .await?;
    let iteration: u8 = m.body().unwrap();
    println!("{}", iteration + 1);
    Ok(())
}

async fn get_remaining(connection: &Connection) -> Result<(), Box<dyn Error>> {
    let m = connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "GetRemaining",
            &(),
        )
        .await?;
    let remaining: Duration = m.body().unwrap();
    let remaining_secs = remaining.as_secs();
    println!("{:02}:{:02}", remaining_secs / 60, remaining_secs % 60);
    Ok(())
}

async fn is_running(connection: &Connection) -> Result<(), Box<dyn Error>> {
    let m = connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "IsRunning",
            &(),
        )
        .await?;
    let is_running: bool = m.body().unwrap();
    println!("{}", is_running as u8);
    Ok(())
}

async fn is_on_break(connection: &Connection) -> Result<(), Box<dyn Error>> {
    let m = connection
        .call_method(
            Some("dev.exvacuum.pomd"),
            "/dev/exvacuum/pomd",
            Some("dev.exvacuum.pomd"),
            "IsOnBreak",
            &(),
        )
        .await?;
    let is_on_break: bool = m.body().unwrap();
    println!("{}", is_on_break as u8);
    Ok(())
}