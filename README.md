# solana-voting

:white_check_mark: Two vote topics ('Next Class president for 2023' and 'Next president for 2022') were created here by the admin with various options separated by comma. :ok_woman:, :ok_person:, :ok_man:

![alt text](https://github.com/maars202/solana-voting/blob/main/solana-voting-programs/imgs/voteTopicsCreated.png)

:x: Error was thrown in the solana program when only one option with no commas was provided. It does not make sense for vote topics to be provided with only one option and so this is to prevent admins from creating such voting topics. 

:eight_pointed_black_star: The solana-voting program can also be filtered for vote topics such as 'Next president for 2022' which is shown below with its respective options.

![alt text](https://github.com/maars202/solana-voting/blob/main/solana-voting-programs/imgs/errorForOneOption.png)



STEPS for using sol-stream-voting API to get logs from solana program:

open 4 terminals

Terminal #1:
```solana config set --url localhost
solana-test-validator --no-bpf-jit --reset
```

Terminal #2:
```solana logs```

Terminal #3:
set sol-stream-voting/.env to postgresql database url
```cd sol-stream-voting 
cargo run```

click on http://127.0.0.1:8000 to start a thread for listening to solana program events

Terminal #4:
```
cd solana-voting-programs
anchor build
anchor deploy```

Take note of programId displayed:

place programId in solana-voting-programs/programs/solana-voting/src/lib.rs in line 7 in ... declare_id!("85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ") 
place programId in solana-voting-programs/Anchor.toml in solana_voting = "85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ"

```anchor run test2```
After the test have been run, it will generate logs related to the program you have deployed on the solana local testnet.

You should be able to see these program logs in the API terminal and the postgresql table 'logstreams'.

