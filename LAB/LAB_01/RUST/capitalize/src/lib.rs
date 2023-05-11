mod main;

#[cfg(test)]
mod test {

    use crate::main::main_conv::conversion;

    #[test]
    fn test_1() {
        let user = "questa è una frase".to_string();
        let result = "Questa È Una Frase";
        assert_eq!(conversion(user), result);
    }

    #[test]
    fn test_2() {
        let user = "questa".to_string();
        let result = "Questa";
        assert_eq!(conversion(user), result);
    }

    #[test]
    fn test_3() {
        let user = "è una frase".to_string();
        let result = "È Una Frase";
        assert_eq!(conversion(user), result);
    }

    #[test]
    fn test_4() {
        let user = "".to_string();
        let result = "";
        assert_eq!(conversion(user), result);
    }

    #[test]
    fn test_5() {
        let user = "questa  è una frase".to_string();
        let result = "Questa  È Una Frase";
        assert_eq!(conversion(user), result);
    }
}
