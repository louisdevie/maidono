#[macro_export]
macro_rules! problem {
    ($s: literal) => {
        $crate::utils::Error::StaticMessage($s)
    };

    ($e: expr) => {
        $crate::utils::Error::from($e)
    };

    ($f: expr, $($args:expr)*) => {
    	$crate::utils::Error::DynamicMessage(format!($f, $($args)*))
    }
}
