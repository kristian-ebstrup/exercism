#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }
        else {
            let position = ChessPosition {
                rank,
                file,
            };
            return Some(position);
        };
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        let queen = Queen {
            position,
        };
        return queen
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let positional_difference: Option<ChessPosition> = ChessPosition::new(
            (&self.position.rank - other.position.rank).abs(),
            (&self.position.file - other.position.file).abs(),
            );
        
        match positional_difference {
            None => return false,
            Some(difference) => {
                if difference.rank == 0 || difference.file == 0 || difference.rank == difference.file {
                    return true;
                }
                else {
                    return false;
                }
            }
        }
    }
}
