use std::{fs::File, io::Error};

/// Cast from one type into another

pub struct MB {
  val: String  
}

impl From<String> for MB {
    fn from(a: String) -> Self {
        MB { val: a }
    }
}


fn mk_mb(a: String) -> MB {
    // because of the From trait we are able to "auto" cast the String to MB
    a.into()
}


/// Cast Error to custom Error
/// 

struct CustomError<'a> {
    a: String,
    msg: &'a str
}

impl<'a> From<std::io::Error> for CustomError<'a> {
    fn from(err: std::io::Error) -> Self {
        CustomError { a: err.to_string(), msg: "my message" }
    }
}

fn do_mess_with_custom_error<'a>() -> Result<(), CustomError<'a>>  {
    // this will fire and auto cast into custom error
    let f = File::open("nowhere")?; 

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{mk_mb, do_mess_with_custom_error};

    #[test]
    fn test_cast_into_mb() {
        let a = mk_mb("Hello World".to_string());
        assert_eq!(a.val, "Hello World".to_string());
    }

    #[test]
    fn custom_error_test() {
        let e = do_mess_with_custom_error();
        let p = e.as_ref().err().unwrap();
        assert_eq!(e.is_err(), true);
        assert_eq!(p.msg, "my message");
        assert_eq!(p.a, "No such file or directory (os error 2)");
    }
}