use std::time::Duration;

async fn ex1() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    String::from("Hello from Tokio!")
}

async fn ex2() -> String {
    let task1 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        String::from("Hello")
    });

    let task2 = tokio::spawn(ex2_second_task());

    //TODO: await both tasks concurrently (hint: tokio::join!())
    //TODO: return both messages as combined string (hint: format!())
    let (result1, result2) = tokio::join!(task1, task2);

    format!("{} {}", result1.unwrap(), result2.unwrap())
}

async fn ex2_second_task() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    String::from("there!")
}

async fn ex3() -> String {
    let response = reqwest::get("https://m.media-amazon.com/images/S/msdev-rouleur-test/bruce.jpg")
        .await
        .unwrap();

    // Convert the status code to a String and return it
    response.status().to_string()
}

#[tokio::main]
async fn main() {
    let result1 = ex1();
    let result2 = ex2();
    let result3 = ex3();

    //TODO: hmmm, something does not seem right.
    println!(
        "Result 1: {}, Result 2: {}, Result 3: {}", //do not change this line
        result1.await,
        result2.await,
        result3.await
    );
}
