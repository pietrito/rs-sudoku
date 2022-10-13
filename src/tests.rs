#[cfg(test)]
mod tests {
    use crate::game;

    #[test]
    fn test_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_valids() {
        let mut game = game::Game::new(3, None).unwrap();
        let mut valids = game.valids(0);
        valids.sort();
        assert_eq!(valids, (1..=9).collect::<Vec<u8>>());

        game.do_move(0, 0, 9).unwrap();
        let mut valids = game.valids(0);
        valids.sort();
        assert_eq!(valids, (1..=8).collect::<Vec<u8>>());
    }

    #[test]
    fn test_nb_empty() {
        let mut game = game::Game::new(3, None).unwrap();
        assert_eq!(game.nb_empty(), 81);
        game.grid = vec![
            game::Cell {
                value: 1,
                initial: false
            };
            81
        ];
        assert_eq!(game.nb_empty(), 0);
    }

    #[test]
    fn test_nb_non_empty() {
        let mut game = game::Game::new(3, None).unwrap();
        assert_eq!(game.nb_non_empty(), 0);
        game.grid = vec![
            game::Cell {
                value: 1,
                initial: false
            };
            81
        ];
        assert_eq!(game.nb_non_empty(), 81);
    }

    #[test]
    fn test_coordinates() {
        let game = game::Game::new(3, None).unwrap();
        assert_eq!(game.coordinates(0), (0, 0));
        assert_eq!(game.coordinates(9), (1, 0));
    }
}
