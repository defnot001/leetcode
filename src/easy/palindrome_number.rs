// my approach
pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    let s = x.to_string();
    let reversed = x.to_string().chars().rev().collect::<String>();

    s == reversed
}

// taken from solutions -> One-liner
pub fn is_palindrome_smaller(x: i32) -> bool {
    return x.to_string().chars().rev().eq(x.to_string().chars());
}
