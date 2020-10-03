
pub fn add(a:i32,b:i32)->i32{
    a+b
}

pub fn add2(a:i32,b:i32)->Result<i32,String>{
    if (a+b)>1
    {
        Ok(a+b)
    }   
    else
    {
        panic!("Fail")
    }   
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(add(1,2),3);
    }

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
    #[test]
    #[should_panic]
    fn test2() {
       add2(5,0);
    }
}

fn main() {
    println!("testing");
}
