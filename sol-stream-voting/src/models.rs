use crate::diesel::ExpressionMethods;
use borsh::{BorshDeserialize, BorshSerialize};
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::Serialize;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::pubkey::Pubkey;
use chrono::Utc;

// use crate::schema::streams;
use crate::schema::logstreams;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
struct StreamData {
    pub start_time: UnixTimestamp,
    pub end_time: UnixTimestamp,
    pub receiver: Pubkey,
    pub lamports_withdrawn: u64,
    pub amount_second: u64,
    pub sender: Pubkey,
}


#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
struct StreamData2 {
// #[account]
// pub struct Votetopic {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub options: String,
    pub voters: Vec<String>,
    // pub voters: Vec<Pubkey>,
    // pub voters: Vec<Voter>,
// }

}

// #[derive(Queryable, Insertable, Serialize)]
// #[table_name = "streams"]
#[derive(Debug)]
pub struct Stream {
    pub pda_account: String,
    pub start_time: i64,
    pub end_time: i64,
    pub receiver: String,
    pub lamports_withdrawn: i64,
    pub amount_second: i64,
    pub sender: String,
    pub total_amount: i64,
}

impl Stream {
    pub fn new(pda_pubkey: String, pda_data: &Vec<u8>) -> Option<Self> {
        // let stream_data = match StreamData::try_from_slice(pda_data) {
            let stream_data = match StreamData::try_from_slice(pda_data) {
            Ok(a) => a,
            Err(e) => {
                println!(
                    "Failed to deserialize {} with error {:?}",
                    pda_pubkey.to_string(),
                    e
                );
                return None;
            }
        };

        Some(Stream {
            sender: stream_data.sender.to_string(),
            end_time: stream_data.end_time,
            receiver: stream_data.receiver.to_string(),
            lamports_withdrawn: stream_data.lamports_withdrawn as i64,
            start_time: stream_data.start_time,
            total_amount: (stream_data.end_time - stream_data.start_time)
                * stream_data.amount_second as i64,
            pda_account: pda_pubkey,
            amount_second: stream_data.amount_second as i64,
        })
    }

    // pub fn get_all_with_sender(pubkey: &String, conn: &PgConnection) -> Vec<Stream> {
    //     use crate::schema::streams::dsl::*;
    //     streams
    //         .filter(sender.eq(pubkey))
    //         .load::<Stream>(conn)
    //         .unwrap()
    // }

    // pub fn get_all_with_receiver(pubkey: &String, conn: &PgConnection) -> Vec<Stream> {
    //     use crate::schema::streams::dsl::*;
    //     streams
    //         .filter(receiver.eq(pubkey))
    //         .load::<Stream>(conn)
    //         .unwrap()
    // }

    // fn id_is_present(id: &String, conn: &PgConnection) -> bool {
    //     use crate::schema::streams::dsl::*;
    //     match streams.find(id).first::<Stream>(conn) {
    //         Ok(_s) => true,
    //         _ => false,
    //     }
    // }
    // pub fn insert_or_update(stream: Stream, conn: &PgConnection) -> bool {
    //     if Stream::id_is_present(&stream.pda_account, conn) {
    //         use crate::schema::streams::dsl::{
    //             amount_second as a_s, end_time as e_t, lamports_withdrawn as l_w,
    //             pda_account as p_a, receiver as r, sender as s, streams, total_amount as t_a,
    //         };
    //         diesel::update(streams.filter(p_a.eq(stream.pda_account)))
    //             .set((
    //                 a_s.eq(stream.amount_second),
    //                 r.eq(stream.receiver),
    //                 s.eq(stream.sender),
    //                 l_w.eq(stream.lamports_withdrawn),
    //                 t_a.eq(stream.total_amount),
    //                 e_t.eq(stream.end_time),
    //             ))
    //             .execute(conn)
    //             .is_ok()
    //     } else {
    //         diesel::insert_into(crate::schema::streams::table)
    //             .values(&stream)
    //             .execute(conn)
    //             .is_ok()
    //     }
    // }
}


// pub author: Pubkey,
// pub timestamp: i64,
// pub topic: String,
// pub options: String,
// pub voters: Vec<String>,

// pub start_time: UnixTimestamp,
// pub end_time: UnixTimestamp,
// pub receiver: Pubkey,
// pub lamports_withdrawn: u64,
// pub amount_second: u64,
// pub sender: Pubkey,

#[derive(Debug)]
pub struct Stream2 {
    // pub pda_account: String,
    // pub start_time: i64,
    // pub end_time: i64,
    // pub receiver: String,
    // pub lamports_withdrawn: i64,
    // pub amount_second: i64,
    // pub sender: String,
    // pub total_amount: i64,

    pub author: String,
    pub timestamp: i64,
    pub topic: String,
    pub options: String,
    pub voters: Vec<String>,
}



impl Stream2 {
    pub fn new(pda_pubkey: String, pda_data: &Vec<u8>) -> Option<Self> {
        // let stream_data = match StreamData::try_from_slice(pda_data) {
            let stream_data = match StreamData2::try_from_slice(pda_data) {
            Ok(a) => a,
            Err(e) => {
                println!(
                    "Failed to deserialize {} with error {:?}",
                    pda_pubkey.to_string(),
                    e
                );
                return None;
            }
        };

        Some(Stream2 {
            // sender: stream_data.sender.to_string(),
            // end_time: stream_data.end_time,
            // receiver: stream_data.receiver.to_string(),
            // lamports_withdrawn: stream_data.lamports_withdrawn as i64,
            // start_time: stream_data.start_time,
            // total_amount: (stream_data.end_time - stream_data.start_time)
            //     * stream_data.amount_second as i64,
            // pda_account: pda_pubkey,
            // amount_second: stream_data.amount_second as i64,

            author: stream_data.author.to_string(),
            timestamp: stream_data.timestamp,
            topic: stream_data.topic.to_string(),
            options: stream_data.options.to_string(),
            voters: stream_data.voters,

        })
    }

}

// #[derive(Debug)]
#[derive(Queryable, Insertable, Serialize, Debug)]
#[table_name = "logstreams"]
pub struct LogStream {
    pub logs: String,
    pub timestored: i64,
    pub displayed: bool,
   
}

impl LogStream {
    pub fn new(pda_data: Vec<String>) -> Option<Self> {
        // let stream_data = match StreamData::try_from_slice(pda_data) {
        //     let stream_data = match StreamData2::try_from_slice(pda_data) {
        //     Ok(a) => a,
        //     Err(e) => {
        //         println!(
        //             "Failed to deserialize {:?} with error {:?}",
        //             pda_data,
        //             e
        //         );
        //         return None;
        //     }
        // };
        let joined = pda_data.join("|||");
        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        Some(LogStream {
            logs: joined,
            timestored: timestamp,
            displayed: false,
        })

    }

    pub fn insert_or_update(stream: LogStream, conn: &PgConnection) -> bool {
        println!("INSERTING HEREEEE*************************************");
        // if Stream::id_is_present(&stream.pda_account, conn) {
        //     use crate::schema::streams::dsl::{
        //         amount_second as a_s, end_time as e_t, lamports_withdrawn as l_w,
        //         pda_account as p_a, receiver as r, sender as s, streams, total_amount as t_a,
        //     };
        //     diesel::update(streams.filter(p_a.eq(stream.pda_account)))
        //         .set((
        //             a_s.eq(stream.amount_second),
        //             r.eq(stream.receiver),
        //             s.eq(stream.sender),
        //             l_w.eq(stream.lamports_withdrawn),
        //             t_a.eq(stream.total_amount),
        //             e_t.eq(stream.end_time),
        //         ))
        //         .execute(conn)
        //         .is_ok()
        // } else {
            diesel::insert_into(crate::schema::logstreams::table)
                .values(&stream)
                .execute(conn)
                .is_ok()
                // .expect("Error saving new post")
        // }
    }
}