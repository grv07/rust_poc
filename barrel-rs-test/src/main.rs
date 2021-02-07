use barrel::{types, Migration};
use barrel::backend::MySql;

fn main() {
    let mut m = Migration::new();

    m.create_table("users", |t| {
        t.add_column("name", types::varchar(255));
        t.add_column("age", types::integer());
        t.add_column("owns_plushy_sharks", types::boolean());
    });

    println!("{}", m.make::<MySql>());
}
