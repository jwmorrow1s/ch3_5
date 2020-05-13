#[allow(dead_code)]
fn cardinality(n: usize) -> String {
    match n {
        12 => String::from("twelfth"),
        11 => String::from("eleventh"),
        10 => String::from("tenth"),
        9 => String::from("ninth"),
        8 => String::from("eighth"),
        7 => String::from("seventh"),
        6 => String::from("sixth"),
        5 => String::from("fifth"),
        4 => String::from("fourth"),
        3 => String::from("third"),
        2 => String::from("second"),
        1 => String::from("first"),
        _ => panic_not_twelve_days(),
    }
}

#[allow(dead_code)]
fn panic_not_twelve_days() -> ! {
    panic!("there are 12 days of christmas.")
}

#[allow(dead_code)]
fn day_of_christmas(n: usize) -> String {
    if !(1..=12).contains(&n) {
        panic_not_twelve_days();
    }

    format!(
        "On the {} day of Christmas\n{}",
        cardinality(n),
        "my true love gave to me:\n"
    )
}

#[allow(dead_code)]
fn rest_of_it(n: usize) -> String {
    if !(1..=12).contains(&n) {
        panic_not_twelve_days();
    }

    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}.",
        if n == 12 {
            "twelve drummers drumming,\n"
        } else {
            ""
        },
        if n >= 11 {
            "eleven pipers piping,\n"
        } else {
            ""
        },
        if n >= 10 {
            "ten lords a-leaping,\n"
        } else {
            ""
        },
        if n >= 9 { "nine ladies dancing,\n" } else { "" },
        if n >= 8 {
            "eight maids a-milking,\n"
        } else {
            ""
        },
        if n >= 7 {
            "seven swans a-swimming,\n"
        } else {
            ""
        },
        if n >= 6 { "six geese a-laying,\n" } else { "" },
        if n >= 5 { "five golden rings,\n" } else { "" },
        if n >= 4 { "four calling birds,\n" } else { "" },
        if n >= 3 { "three french hens,\n" } else { "" },
        if n >= 2 { "two turtledoves,\n" } else { "" },
        if n > 1 {
            "and a partridge in a pear tree"
        } else {
            "a partridge in a pear tree"
        },
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cardinality_it_works() {
        assert_eq!(cardinality(1), "first");
        assert_eq!(cardinality(2), "second");
        assert_eq!(cardinality(3), "third");
        assert_eq!(cardinality(4), "fourth");
        assert_eq!(cardinality(5), "fifth");
        assert_eq!(cardinality(6), "sixth");
        assert_eq!(cardinality(7), "seventh");
        assert_eq!(cardinality(8), "eighth");
        assert_eq!(cardinality(9), "ninth");
        assert_eq!(cardinality(10), "tenth");
        assert_eq!(cardinality(11), "eleventh");
        assert_eq!(cardinality(12), "twelfth");
    }

    #[test]
    #[should_panic]
    fn cardinality_invalid_zero_should_panic() {
        cardinality(0);
    }

    #[test]
    #[should_panic]
    fn cardinality_invalid_gt_twelve_should_panic() {
        cardinality(13);
    }

    #[test]
    fn day_of_christmas_it_works() {
        assert_eq!(
            day_of_christmas(12),
            "On the twelfth day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(11),
            "On the eleventh day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(10),
            "On the tenth day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(9),
            "On the ninth day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(8),
            "On the eighth day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(7),
            "On the seventh day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(6),
            "On the sixth day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(5),
            "On the fifth day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(4),
            "On the fourth day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(3),
            "On the third day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(2),
            "On the second day of Christmas\nmy true love gave to me:\n"
        );
        assert_eq!(
            day_of_christmas(1),
            "On the first day of Christmas\nmy true love gave to me:\n"
        );
    }

    #[test]
    #[should_panic]
    fn day_of_christmas_zero_days() {
        day_of_christmas(0);
    }

    #[test]
    #[should_panic]
    fn day_of_christmas_gt_twelve_days() {
        day_of_christmas(13);
    }

    #[test]
    fn rest_of_it_full_it_works() {
        assert_eq!(
            rest_of_it(12),
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}",
                "twelve drummers drumming,\n",
                "eleven pipers piping,\n",
                "ten lords a-leaping,\n",
                "nine ladies dancing,\n",
                "eight maids a-milking,\n",
                "seven swans a-swimming,\n",
                "six geese a-laying,\n",
                "five golden rings,\n",
                "four calling birds,\n",
                "three french hens,\n",
                "two turtledoves,\n",
                "and a partridge in a pear tree."
            )
        );
    }

    #[test]
    fn rest_of_it_one_it_works() {
        assert_eq!(
            rest_of_it(1),
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "a partridge in a pear tree."
            )
        );
    }
}
