use adder;

#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        let result = adder::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    // fn fail() {
    //     panic!("fail on purpose   ")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = adder::Rectangle {
            width: 8,
            height: 7
        };

        let smaller = adder::Rectangle {
            width: 6,
            height: 4
        };

        assert!(larger.can_hold(&smaller))
    }
}
