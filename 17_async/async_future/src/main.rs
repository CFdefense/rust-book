use trpl::{Either, Html};

async fn page_title(url: &str) -> Option<String> {
    // we await the url
    let response = trpl::get(url).await;

    // then we await the text of the
    let response_text = response.text().await;

    // then we can handle the response and attempt to parse the <title> element
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    // collect cli arguments
    let args: Vec<String> = std::env::args().collect();

    // use block_on to initalize a runtime
    trpl::block_on(async {
        // call page title for each url
        let title_fut_1 = async { (&args[1], page_title(&args[1]).await) };
        let title_fut_2 = async { (&args[2], page_title(&args[2]).await) };

        // match the results of select
        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        // print who finished first
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}
