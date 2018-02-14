extern crate uuid;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_yaml;

use std::str::FromStr;

mod accounting;
mod app;
mod registry;
mod bot;
mod persistence;
mod error;
mod config;

use app::{App, Command, EntryCmd, MigrateCmd, UserCmd};
use registry::Registry;
use accounting::{Entry, Product, TelegramId};
use error::{Error};

fn main() {
    env_logger::init();
    let app = match App::from_args() {
        Ok(app) => app,
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        }
    };
    info!("{:?}", &app);

    match start(app) {
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        },
        Ok(_) => {}
    };    
}

fn start(app: App) -> Result<(), Error> {
    let config = config::config(&app)?;
    info!("config: {:?}", &config);
    let registry = Registry::new(config.data_path.clone().into())?;

    match app.command {
        Command::Entry(EntryCmd::List) => {
            for entry in registry.list()? {
                println!("{}", accounting::representation::EntryRepresentation::from(entry));
            }
        },
        Command::Entry(EntryCmd::Add(new_entry)) => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            let parsed_new_product = Product::from_str(&new_entry)?;
            let new_entry = Entry::new(user.id, parsed_new_product);
            registry.add_entry(new_entry)?;
        },
        Command::Bot => {
            bot::BotLauncher::new(registry, config).start()?;
        },
        Command::Migrate(MigrateCmd::Add(field_name, value)) => {
            registry.migrate_entries(::persistence::Migration::add_from_str(field_name, &value)?)?;
        },
        Command::Migrate(MigrateCmd::Remove(field_name)) => {
            registry.migrate_entries(::persistence::Migration::remove(field_name))?;
        },
        Command::User(UserCmd::List) => {
            for user in registry.list_users()? {
                println!("{:?}", user);
            }
        },
        Command::User(UserCmd::Add(user_telegram_id)) => {
            let user = registry.find_or_create(TelegramId(user_telegram_id))?;
            println!("{:?}", user);
        }
    }

    Ok(())
}
