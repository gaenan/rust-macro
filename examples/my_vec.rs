use anyhow::Result;

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),+]))
        }
    };
}

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![1;4];
    let v1: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
    ];

    // let v = vec![1, 2, 3];

    let v2 = <[_]>::into_vec(Box::new([1, 2, 3, 4]));

    println!("first --> {:?}", v);

    println!("second --> {:?}", v1);

    println!("third --> {:?}", v2);

    Ok(())
}
