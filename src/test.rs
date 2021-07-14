
#[cfg(test)]
mod test {
    use crate::{add_days, calculate_pax, correct_date, get_supplementary_files};

    //===============================================================================================================//

    #[test]
    fn test_get_supplementary_files() {
        get_supplementary_files(String::from("hotels.json"), String::from("room_names.csv"))
            .unwrap();
    }

    //===============================================================================================================//

    #[test]
    #[should_panic]
    fn test_panic_get_supplementary_files() {
        get_supplementary_files(String::from("sletoh.json"), String::from("seman_moor.csv"))
            .unwrap();
    }

    //===============================================================================================================//

    #[test]
    fn test_correct_date() {
        let mut date = String::from("20010914");
        correct_date(&mut date);

        assert!(String::from("20010914") != date);
        assert!(String::from("2001-09-14") == date);
    }

    //===============================================================================================================//

    #[test]
    fn test_add_days() {
        let mut date = String::from("20010914");
        correct_date(&mut date);

        assert!(String::from("20010914") != date);
        assert!(String::from("2001-09-14") == date);

        let day = add_days(&date, 1).unwrap();
        assert!(String::from("2001-09-15") == day);
    }

    //===============================================================================================================//

    #[test]
    #[should_panic]
    fn test_panic_add_days() {
        let mut date = String::from("20010914");
        correct_date(&mut date);

        assert!(String::from("20010914") != date);
        assert!(String::from("2001-09-14") == date);

        let day = add_days(&date, 1).unwrap();
        assert!(String::from("2001-09-15") == day);
        assert!(String::from("20010915") == day);
    }

    //===============================================================================================================//

    #[test]
    fn test_calculate_pax() {
        let pax = calculate_pax(20, 20);
        assert!(pax == 40);

        let pax = calculate_pax(std::u32::MAX, std::u32::MAX);
        assert!(pax == 8589934590);
    }

    //===============================================================================================================//
}
