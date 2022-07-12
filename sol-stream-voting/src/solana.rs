use std::{str::FromStr, thread};

use solana_client::{pubsub_client, rpc_client::RpcClient};
use solana_sdk::{account::Account, pubkey::Pubkey};


use solana_client::rpc_config::RpcTransactionLogsFilter;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_client::rpc_config::RpcTransactionLogsConfig;

use crate::{establish_connection, models::Stream, models::Stream2, models::LogStream,};
// use crate::{establish_connection};

fn get_all_program_accounts() -> Vec<(Pubkey, Account)> {
    // 7sQC1QExqkVZBgxnP8ra25NgCrqe4rQSwTThtVuV9zqk
    let program_pub_key = Pubkey::from_str("85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ")
        .expect("program address invalid");
        // https://docs.rs/solana-client/1.7.11/solana_client/rpc_client/struct.RpcClient.html
    // let url = "https://api.devnet.solana.com".to_string();
    // let url = "http://localhost:8899".to_string();
    let url = "http://localhost:8899".to_string();
    // let url = "ws://localhost:8900".to_string();
    let client = RpcClient::new(url);

    client
        .get_program_accounts(&program_pub_key)
        .expect("Something went wrong here get_all_program_accounts")
}

pub fn subscribe_to_program() {
    // let url = "ws://api.devnet.solana.com".to_string();
    let url = "ws://localhost:8900".to_string();
    // let url = "http://localhost:8899".to_string();
    // 7sQC1QExqkVZBgxnP8ra25NgCrqe4rQSwTThtVuV9zqk
    let program_pub_key = Pubkey::from_str("85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ")
        .expect("program address invalid");

    thread::spawn(move || loop {
        let subscription =
            pubsub_client::PubsubClient::program_subscribe(&url, &program_pub_key, None)
                .expect("Something went wrong subscribe_to_program");

                // connect to database:
        // let conn = establish_connection();

        loop {
            println!("helloooo");
            let response = subscription.1.recv();
            println!("this is the response: {:#?}", response);
            match response {
                Ok(response) => {
                    let pda_pubkey = response.value.pubkey;
                    let pda_account: Account = response.value.account.decode().unwrap();

                    let stream = Stream2::new(pda_pubkey, &pda_account.data);
                    match stream {
                        Some(a) => {
                            println!("1. inserting into table happening here, a: {:#?}", a);
                            continue;
                            // Stream::insert_or_update(a, &conn)
                        },
                        _ => {
                            println!("data didn't parsed");
                            continue;
                        }
                    };
                }
                Err(_) => {
                    break;
                }
            }
        }
        get_accounts_and_update()
    });
}


pub fn subscribe_to_program2() {
    let url = "ws://localhost:8900".to_string();
    let program_pub_key = Pubkey::from_str("85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ")
        .expect("program address invalid");

    thread::spawn(move || loop {
        // let subscription =
        //     pubsub_client::PubsubClient::program_subscribe(&url, &program_pub_key, None)
        //         .expect("Something went wrong subscribe_to_program");
        let subscription =
            pubsub_client::PubsubClient::logs_subscribe(&url, 
                // RpcTransactionLogsFilter.ALL, 
                RpcTransactionLogsFilter::All,
                // RpcTransactionLogsConfig(Some(CommitmentConfig::recent)))
                RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::recent()) })
                .expect("Something went wrong subscribe_to_program");

             // connect to database:
        let conn = establish_connection();
        loop {
            println!("2helloooo2");
            let response = subscription.1.recv();
            println!("2this is the response: {:#?}", response);
            match response {
                Ok(response) => {
                    let pda_account: Vec<String> = response.value.logs;
                    let mut filteredLogs: Vec<String> = vec![];
                    for text in &pda_account {
                        if text.starts_with(&String::from("Program BPFLoaderUpgradeab")){
                            println!("Ignoring {}.", text);
                        }else{
                            filteredLogs.push(text.to_string());
                        }
                        
                    }

                    if filteredLogs.len() == 0{
                        continue;
                    }
                    // "Program BPFLoaderUpgradeab1e11111111111111111111111"
                    // let joined = filteredLogs.join("|||");
                    // println!("logsss: {:#?}", joined);

                    let stream = LogStream::new(filteredLogs);
                    match stream {
                        Some(a) => {
                            println!("1. inserting into table happening here, a: {:#?}", a);
                            
                            // Stream::insert_or_update(a, &conn)
                            let result = LogStream::insert_or_update(a, &conn);
                            if result != true{
                                println!("it didnt get inserted!");
                            }
                            continue;
                        },
                        _ => {
                            println!("data didn't parsed");
                            continue;
                        }
                    };
                }
                Err(_) => {
                    println!("error!");
                    break;
                }
            }
        }
        // get_accounts_and_update()
    });
}

pub fn subscribe_to_program3() {
    let url = "ws://localhost:8900".to_string();
    let program_pub_key = Pubkey::from_str("85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ")
        .expect("program address invalid");

    thread::spawn(move || loop {
        let subscription =
            pubsub_client::PubsubClient::program_subscribe(&url, &program_pub_key, None)
                .expect("Something went wrong subscribe_to_program");
        loop {
            println!("2helloooo2");
            let response = subscription.1.recv();
            println!("2this is the response: {:#?}", response);
            match response {
                Ok(response) => {
                    let pda_pubkey = response.value.pubkey;
                    let pda_account: Account = response.value.account.decode().unwrap();


                    let stream = Stream2::new(pda_pubkey, &pda_account.data);
                    match stream {
                        Some(a) => {
                            println!("1. inserting into table happening here, a: {:#?}", a);
                            // continue;
                            // Stream::insert_or_update(a, &conn)
                            break;
                        },
                        _ => {
                            println!("data didn't parsed");
                            // continue;
                            break;
                        }
                    };
                }
                Err(_) => {
                    println!("error!");
                    break;
                }
            }
        }
        // get_accounts_and_update()
    });
}



pub fn get_accounts_and_update() {
    println!("get_accounts_and_update method happening!");
    let program_accounts = get_all_program_accounts();
    // let conn = establish_connection();
    for item in program_accounts.iter() {
        let stream = Stream2::new(item.0.to_string(), &item.1.data);
        match stream {
            // Some(a) => Stream::insert_or_update(a, &conn),
            Some(a) => {
                println!("2. inserting into table happening here, a: {:#?}", a);
                continue
            },
            _ => continue,
        };
    }
}