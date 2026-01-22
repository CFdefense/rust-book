use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // create our channel
        let (tx, mut rx) = trpl::channel();

        // clone into tx1
        let tx1 = tx.clone();

        // create an asycn block and move values used (tx1)
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            // send and sleep each val
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // recieve the values
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // now create another async block and move values used (tx)
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            // send and sleep each val
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // join the futures
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}
