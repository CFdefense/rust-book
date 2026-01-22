use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        // create an array of values
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // convert to iterator and map with a closure to double each
        let iter = values.iter().map(|n| n * 2);

        // convert iter to a stream
        let mut stream = trpl::stream_from_iter(iter);

        // await the stream values using stream .next()
        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
    }
    });
}
