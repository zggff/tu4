mod machine;
mod tape;

pub use machine::{Machine, ParseTu4Error};
pub use tape::Tape;

#[cfg(test)]
mod tests {
    use crate::{Machine, ParseTu4Error};
    #[test]
    fn test_parsing() {
        let code = "00, , ,00".parse::<Machine>();
        assert!(code.is_ok());

        let code = "00, , ".parse::<Machine>();
        assert_eq!(code.err(), Some(ParseTu4Error::InvalidFormat));

        let code = "00,cc, ,00".parse::<Machine>();
        assert_eq!(code.err(), Some(ParseTu4Error::InvalidCellValue));

        let code = "00, ,cc,00".parse::<Machine>();
        assert_eq!(code.err(), Some(ParseTu4Error::InvalidCellValue));

        let code = "00, , ,-00".parse::<Machine>();
        assert_eq!(
            code.err(),
            "-00"
                .parse::<usize>()
                .map_err(ParseTu4Error::ParseInt)
                .err()
        );

        let code = "00, , ,01".parse::<Machine>();
        assert_eq!(code.err(), Some(ParseTu4Error::InvalidTransition(1)));
    }

    #[test]
    fn test_execution() {
        let code = "00, ,<,01
                    01,0,<,01
                    01,1,<,01
                    01, ,>,02
                    03,0,>,02
                    03,1,>,02
                    02,0, ,04
                    04, ,>,05
                    05,0,>,05
                    05,1,>,05
                    05, ,>,06
                    06,0,>,06
                    06,1,>,06
                    06, ,0,07
                    07,0,<,07
                    07,1,<,07
                    07, ,<,08
                    08,0,<,08
                    08,1,<,08
                    08, ,0,03
                    02,1, ,09
                    09, ,>,10
                    10,0,>,10
                    10,1,>,10
                    10, ,>,11
                    11,0,>,11
                    11,1,>,11
                    11, ,1,12
                    12,0,<,12
                    12,1,<,12
                    12, ,<,13
                    13,0,<,13
                    13,1,<,13
                    13, ,1,03
                    02, , ,02
                    ";
        let machine: Result<Machine, _> = code.parse();
        assert!(machine.is_ok());
        let mut machine = machine.unwrap();
        machine.set_input("110100");
        assert_eq!(machine.tape().to_string(), "110100_");
        machine.execute();
        assert_eq!(machine.tape().to_string(), "_110100_110100");
    }
}
