use clap::{Arg, App, SubCommand};
use git;
use encrypt;
use decrypt;
use make_signed_user_payment;
use publish;
use user_transactions;
use user;
use users;
use merchant;
use merchants;
use create_user;
use create_merchant;
use credit_merchant;
use transaction;
use transactions;
use make_transaction;
use define_merchant_hierarchy;
use make_transaction_to_merchant;



fn main() {

    const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let matches =  
    App::new("eDeposite Cryptocurrency")
        .version(APP_VERSION)
        .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
        .about("Core of eDeposite cryptocurrency")
        
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            //.value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
        )


        .arg(Arg::with_name("hamdan")
            .short("x")
            .long("hamdan")
            //.value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
        )
                          
            
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity")
        )



        .subcommand(App::new("git")
            .about("Run git commands with ease")
            .version("1.0")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")


            .subcommand(App::new("init")
                .about("Initiate git in workspace")
                .version(APP_VERSION)
                .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
            )


            .subcommand(App::new("push")
                .about("Push all changes the main branch")
                .version(APP_VERSION)
                .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
            )


            .subcommand(App::new("commit")
                .about("Commit all changes in the workspace")
                .version(APP_VERSION)
                .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
            )
            
        )

            
        
        .subcommand(SubCommand::with_name("encrypt")
            .about("controls testing features")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )

            .arg(Arg::with_name("sender_public_address")
                .help("Public address of the sender of the token")
            )

            .arg(Arg::with_name("sender_private_key")
                .help("Private key of the sender of the token")
            )

            .arg(Arg::with_name("recipient_public_address")
                .help("Recipient address of the sender of the token")
            )

            
            .arg(Arg::with_name("token")
                .help("Token transferred between users")
            )
            
        )


        .subcommand(SubCommand::with_name("decrypt")
            .about("controls testing features")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )



        .subcommand(SubCommand::with_name("make-signed-user-payment")
            .about("controls testing features")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )



        .subcommand(SubCommand::with_name("publish")
            .about("controls testing features")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
            
        )



        .subcommand(SubCommand::with_name("user")
            .about("Details of a user")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("public address")
                .help("Public address of user")
                .index(1)
            )
            
        )


        .subcommand(SubCommand::with_name("user-transactions")
            .about("controls testing features")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("public address")
                .help("Public address of user")
                .index(1)
            )
            
        )



        .subcommand(SubCommand::with_name("users")
            .about("Show all users")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
            
        )



        .subcommand(SubCommand::with_name("merchant")
            .about("Show merchant information")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")


            .arg(Arg::with_name("public address")
                .help("Public address of the merchant")
                .index(1)
            )
            
        )




        .subcommand(SubCommand::with_name("merchants")
            .about("Show merchants information")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
        )



        .subcommand(SubCommand::with_name("create-user")
            .about("Create user")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )




        .subcommand(SubCommand::with_name("create-merchant")
            .about("Create merchant")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )




        .subcommand(SubCommand::with_name("credit-merchant")
            .about("Credit merchant")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )



        .subcommand(SubCommand::with_name("transaction")
            .about("Show transactions with transaction ID")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
            
            .arg(Arg::with_name("transaction ID")
                .help("Transaction ID")
                .index(1)
            )
        )



        .subcommand(SubCommand::with_name("transactions")
            .about("Show transactions")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")
            
        )



        .subcommand(SubCommand::with_name("make-transaction")
            .about("Make transaction")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )



        .subcommand(SubCommand::with_name("define-merchant-hierarchy")
            .about("define merchant hierarchy")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )



        .subcommand(SubCommand::with_name("make-transaction-to-merchant")
            .about("define make transaction to merchant")
            .version("1.3")
            .author("Akintomiwa Opemipo <ibnakintomiwa@gmail.com>")

            .arg(Arg::with_name("base64 arguments")
                .help("sender_public_address, sender_private_key, recipient_public_address, token")
                .index(1)
            )
            
        )

    .get_matches();
    
    
    

    if let Some(_matches) = matches.subcommand_matches("git") {
        git::main()
    }



    if let Some(matches) = matches.subcommand_matches("encrypt") {
        if matches.is_present("") {
            println!("Printing debug info...");
        } else {
            encrypt::main()
        }
    }


    if let Some(matches) = matches.subcommand_matches("decrypt") {
        if matches.is_present("") {
            println!("Printing debug info...");
        } else {
            decrypt::main()
        }
    }



    if let Some(matches) = matches.subcommand_matches("make-signed-user-payment") {
        if matches.is_present("") {
            println!("Printing debug info...");
        } else {
            make_signed_user_payment::main();
        }
    }



    if let Some(_matches) = matches.subcommand_matches("publish") {
        publish::main();
    }


    if let Some(_matches) = matches.subcommand_matches("user-transactions") {
        user_transactions::main();
    }


    if let Some(_matches) = matches.subcommand_matches("user") {
        user::main();
    }


    if let Some(_matches) = matches.subcommand_matches("users") {
        users::main();
    }


    if let Some(_matches) = matches.subcommand_matches("merchant") {
        merchant::main();
    }


    if let Some(_matches) = matches.subcommand_matches("merchants") {
        merchants::main();
    }


    if let Some(_matches) = matches.subcommand_matches("create-user") {
        create_user::main();
    }


    if let Some(_matches) = matches.subcommand_matches("create-merchant") {
        create_merchant::main();
    }


    if let Some(_matches) = matches.subcommand_matches("credit-merchant") {
        credit_merchant::main();
    }


    if let Some(_matches) = matches.subcommand_matches("transaction") {
        transaction::main();
    }


    if let Some(_matches) = matches.subcommand_matches("transactions") {
        transactions::main();
    }


    if let Some(_matches) = matches.subcommand_matches("make-transaction") {
        make_transaction::main();
    }


    if let Some(_matches) = matches.subcommand_matches("define-merchant-hierarchy") {
        define_merchant_hierarchy::main();
    }


    if let Some(_matches) = matches.subcommand_matches("make-transaction-to-merchant") {
        make_transaction_to_merchant::main();
    }


}