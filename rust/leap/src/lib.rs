pub fn is_leap_year(year: u64) -> bool {
    return year%4 == 0 && (year%100 != 0 || year%400 == 0);
    //unimplemented!("true if {} is a leap year", year)
}
