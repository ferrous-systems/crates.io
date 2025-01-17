use diesel::sql_types::{Date, Double, Integer, Interval, SingleValue, Text, Timestamp};

sql_function!(#[aggregate] fn array_agg<T: SingleValue>(x: T) -> Array<T>);
sql_function!(fn canon_crate_name(x: Text) -> Text);
sql_function!(fn to_char(a: Date, b: Text) -> Text);
sql_function!(fn lower(x: Text) -> Text);
sql_function!(fn date_part(x: Text, y: Timestamp) -> Double);
sql_function! {
    #[sql_name = "date_part"]
    fn interval_part(x: Text, y: Interval) -> Double;
}
sql_function!(fn floor(x: Double) -> Integer);
sql_function!(fn greatest<T: SingleValue>(x: T, y: T) -> T);
sql_function!(fn least<T: SingleValue>(x: T, y: T) -> T);
sql_function!(fn split_part(string: Text, delimiter: Text, n: Integer) -> Text);
