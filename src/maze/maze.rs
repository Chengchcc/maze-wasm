use super::unionFind::{ElementType, UnionFind};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::cell::Cell;
use array_tool::vec::*;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Maze {
    set: UnionFind<u64>,
    cols: u64,
    rows: u64,
    cells: u64,
    linkedMap: HashMap<u64, Vec<u64>>,
}


impl Default for Maze {
    fn default()-> Maze {
        Maze::new(0, 0)
    }
}


impl Maze {
    pub fn new(cols: u64, rows: u64) ->Self {
        let cells: u64 = (cols * rows).into();
        Maze {
            set: UnionFind::new(cells.to_usize()),
            cols: cols,
            rows: rows,
            cells: cells,
            linkedMap: HashMap::new(),
        }
    }


    pub fn reset(&mut self, cols: u64, rows: u64) {
        let cells: u64 = (cols * rows).into();
        self.set = UnionFind::new(cells.to_usize());
        self.cols = cols;
        self.rows = rows;
        self.cells = cells;
        self.linkedMap = HashMap::new();
    }

    pub fn len(&self) -> usize {
        self.cells.to_usize()
    }


    fn pick_random_cell_pairs(&self) -> (u64, u64)  {
        let mut rng = thread_rng();
        let cell: u64 = rng.gen_range(0..self.cells);
        let row: u64 = cell / self.cols as u64;
        let col: u64 = cell % self.rows as u64;

        let mut neiborCells = vec![];

        if row != 0 {
            neiborCells.push(cell - self.cols as u64)
        }

        if row != self.rows as u64 - 1 {
            neiborCells.push(cell + self.cols as u64)
        }

        if col != 0 {
            neiborCells.push(cell - 1)
        }

        if col != self.cols as u64  -1 {
            neiborCells.push(cell + 1)
        }

        let idx= rng.gen_range(0..neiborCells.len());

        (cell, *neiborCells.get(idx).unwrap())
    }

    fn add_linked_map(& mut self, x: u64, y: u64) {
        {
            let arrayX = self.linkedMap.entry(x).or_insert(Vec::new());
            arrayX.push(y);
        }
        let arrayY = self.linkedMap.entry(y).or_insert(Vec::new());
        arrayY.push(x);
    }

    fn linked_to_first_cell(&mut self) -> bool {
        for i in 1..self.cells {
            if !self.set.equiv(0, i ) {
                return false;
            }
        }
        true
    }

    pub fn generate(& mut self) -> &HashMap<u64, Vec<u64>> {
        while ! self.linked_to_first_cell() {
            let (x, y) = self.pick_random_cell_pairs();
            if ! self.set.equiv(x, y) {
                self.set.union(x, y);
                self.add_linked_map(x, y);
            }
        }
        &self.linkedMap
    }

    pub fn cal_path(&mut self) -> Vec<u64> {
        let mut pathTables : Vec<Cell<PathTable<u64>>> = (0..self.cells).map(|_|{
           let p = PathTable {know: false, pre_cell: self.cells};
           Cell::new(p)
        }).collect();
        pathTables[0].get_mut().know = true;

        let mut unserach_cells: Vec<u64> = vec![0];

        while !pathTables[pathTables.len()-1].get().know {
            while unserach_cells.len() > 0 {
                let cell = unserach_cells.pop().unwrap();
                for i in 0..self.linkedMap[&cell].len() {
                    let j  = self.linkedMap[&cell][i];
                    let pathTable  = pathTables[j.to_usize()].get_mut();
                    if !pathTable.know {
                        pathTable.know = true;
                        pathTable.pre_cell = cell;
                        unserach_cells.unshift(j);
                        if pathTables[pathTables.len() -1].get().know {
                            break;
                        }
                    }
                }
            }
        }

        let mut cell = self.cells - 1;
        let mut path = vec![cell];
        while cell != 0 {
            cell = pathTables[cell.to_usize()].get().pre_cell;
            path.push(cell);
        }
        // path.push(cell);
        path
    }

}

#[derive(Copy, Clone, Debug)]
struct PathTable<T: ElementType> {
    know: bool,
    pre_cell: T
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cal_path() {
        let mut maze = Maze::new(25, 25);
        maze.generate();
        let path = maze.cal_path();
         println!("{:?}", path);
    }
}