use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use solana_sdk::pubkey::Pubkey;
// use solana_sdk::transaction::{Transaction, TransactionVersion};

fn read_data_from_bin(path: &str) {
    // read data from local entry.bin and try parse
    let data = std::fs::read(path).unwrap();
    let entries = bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&data).unwrap();

    let mut entry_count = 0;
    for entry in entries {
        println!(
            "num_hashes: {}, hash: {:?}, trx-count: {}",
            entry.num_hashes,
            entry.hash,
            entry.transactions.len()
        );
        for tx in entry.transactions {
            let version = tx.version();
            let version_str = match version {
                solana_sdk::transaction::TransactionVersion::LEGACY => "LEGACY",
                solana_sdk::transaction::TransactionVersion::Number(0) => "V0",
                _ => "UNKNOWN",
            };
            println!("  tx: {:?}, version: {version_str}", tx.signatures[0]);
        }
        entry_count += 1;
    }

    println!("entries: {}", entry_count);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // parse command line arguments
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let path = args[1].clone();
        read_data_from_bin(&path);
        return Ok(());
    }

    let mut client = ShredstreamProxyClient::connect("http://127.0.0.1:20041")
        .await
        .unwrap();
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await
        .unwrap()
        .into_inner();

    // target pubkey to filter
    let target_pubkey = Pubkey::from_str_const("6j5ds1uHR9ai4vdveRdWkrgYF5SRAFG66q4jrMU8LRVv");

    while let Some(slot_entry) = stream.message().await.unwrap() {
        let entries =
            match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                Ok(e) => e,
                Err(e) => {
                    println!("Deserialization failed with err: {e}");
                    continue;
                }
            };

        println!(
            "slot {}, entries: {}, transactions: {}",
            slot_entry.slot,
            entries.len(),
            entries.iter().map(|e| e.transactions.len()).sum::<usize>()
        );

        // set a bool contain version v0
        let mut contains_v0 = false;
        let mut contains_target_pubkey = false;
        for entry in entries {
            for tx in entry.transactions {
                let version = tx.version();
                let _version_str = match version {
                    solana_sdk::transaction::TransactionVersion::LEGACY => "LEGACY",
                    solana_sdk::transaction::TransactionVersion::Number(0) => {
                        contains_v0 = true;
                        "V0"
                    }
                    _ => "UNKNOWN",
                };
                // println!("  tx: {:?}, version: {version_str}", tx.signatures[0]);

                for account in tx.message.static_account_keys() {
                    if account == &target_pubkey {
                        contains_target_pubkey = true;
                        println!("tx {:?} contains target", tx.signatures[0]);
                        break;
                    }
                }
            }
            if contains_v0 {
                println!("slot {}, contains v0", slot_entry.slot);
                // // write the entry to a file
                // let path = format!("entry_{}.bin", slot_entry.slot);
                // std::fs::write(path, &slot_entry.entries).unwrap();
                // return Ok(());
            } else {
                println!("slot {}, does not contain v0", slot_entry.slot);
            }

            let need_to_write = contains_target_pubkey;
            if need_to_write {
                // // write the entry to a file
                let path = format!("entry_{}.bin", slot_entry.slot);
                std::fs::write(path, &slot_entry.entries).unwrap();
                return Ok(());
            }
        }
    }

    Ok(())
}
