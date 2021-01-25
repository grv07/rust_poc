use futures::join;

async fn f_one() {
    println!("{}", 1);
}

async fn f_two() {
    println!("{}", 2);
}

pub fn join_call() {
    async {
        let one = f_one();
        let two = f_two();
        join!(one, two); // concurent run the futures. continue to run even one of future fails.
        // try_join return immidiate if any of passed future fails
    };
}
