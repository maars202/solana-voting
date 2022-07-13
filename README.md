# solana-voting

Firstly, install solana and anchor as shown in the guide https://book.anchor-lang.com/getting_started/installation.html.

:white_check_mark: Two vote topics ('Next Class president for 2023' and 'Next president for 2022') were created here by the admin with various options separated by comma. :ok_woman:, :ok_person:, :ok_man:

![alt text](https://github.com/maars202/solana-voting/blob/main/solana-voting-programs/imgs/voteTopicsCreated.png)

:x: Error was thrown in the solana program when only one option with no commas was provided. It does not make sense for vote topics to be provided with only one option and so this is to prevent admins from creating such voting topics. 

:eight_pointed_black_star: The solana-voting program can also be filtered for vote topics such as 'Next president for 2022' which is shown below with its respective options.

![alt text](https://github.com/maars202/solana-voting/blob/main/solana-voting-programs/imgs/errorForOneOption.png)



## Steps for using sol-stream-voting API to get logs from solana program:

Open 4 terminals

Terminal #1: Start solana local testnet.
```
solana config set --url localhost
solana-test-validator --no-bpf-jit --reset
```

Terminal #2:
```
solana logs
```

Terminal #3:
```
cd solana-voting-programs
anchor build
anchor deploy
```
Take note of programId displayed:
![alt text](https://github.com/maars202/solana-voting/blob/main/solana-voting-programs/imgs/programIDDeployment.png)

Place programId in solana-voting-programs/programs/solana-voting/src/lib.rs in line 7:
```
declare_id!("85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ") 
```

Place programId in solana-voting-programs/Anchor.toml in line 2: 
```
solana_voting = "85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ"
```

Place programId in sol-stream-voting/src/solana.rs in line 16 and 81: 
```
let programID = "85GB2GBrh15nj5vwfPLZBDW4NHqUuWuXeeago9oUEtnJ";
```

Terminal #4: Running diesel API for listening to events
Set sol-stream-voting/.env to postgresql database url.
```
cd sol-stream-voting 
cargo run
```

Go back to Terminal #3:
```
anchor run test2
```
After the test have been run, it will generate logs related to the program you have deployed on the solana local testnet in terminal #4 and terminal #2.

You should be able to see these program logs in the API terminal and the postgresql table 'logstreams'. It will  be similar to sampleProgramLogsPostgresql.csv!

