async fn ex1() -> String {
    //TODO: sleep asynchronously for one second
    todo!();
    String::from("Hello from Tokio!")
}

async fn ex2() -> String {
    let task1 = tokio::spawn(async {
        //sleep for one second and return a message
        todo!()
    });

    let task2 = tokio::spawn(ex2_second_task());

    //TODO: await both tasks concurrently (hint: tokio::join!())
    //TODO: return both messages as combined string (hint: format!())
    todo!()
}

async fn ex2_second_task() -> String {
    //sleep for two seconds, then return a different message
    todo!()
}

async fn ex3() -> String {
    //TODO: Make an HTTP request
    //Hint: https://docs.rs/reqwest/latest/reqwest/fn.get.html
    todo!();

    //TODO: Return the response's status code as a String.
    todo!()
}

#[tokio::main]
async fn main() {
    let result1 = ex1();
    let result2 = ex2();
    let result3 = ex3();

    //TODO: hmmm, something does not seem right.
    println!(
        "Result 1: {}, Result 2: {}, Result 3: {}", //do not change this line
        result1, result2, result3
    );
}
