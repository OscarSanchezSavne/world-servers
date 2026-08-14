use rand::RngExt;
use bevy::{ecs::{component::Component}, math::Vec3};
use uuid::Uuid;


use crate::{core, visualizer};

#[derive(Component, Debug)]
pub struct Grid{
    pub cols: usize,
    pub rows: usize,
    pub matrix_cells: Vec<Vec<visualizer::component::cell::Cell>>,
    pub expanded: bool,
    pub positions: Vec<(usize, usize)>,
}
 
impl Grid {
    
    pub fn create() -> Self {

        let mut positions:Vec<(usize, usize)>= Vec::new();
        let count= core::server::manager::Server::get_servers().len() * 8;
        let count= if count < 30 {30}else{count};
        let mut cols= 3;
        let mut rows= 2;

        if count > 6 {
            // alto * (alto + 1) >= count
            // alto = ceil((-1 + sqrt(1 + 4*count)) / 2)
            let discriminant = 1.0 + 4.0 * count as f64;
            rows = ((-1.0 + discriminant.sqrt()) / 2.0).ceil() as usize;
            if rows < 2 {
                rows = 2;
            }
            cols = rows + 1;
        }
        
        let mut matrix_cells:Vec<Vec<visualizer::component::cell::Cell>> = vec![];

        for row in 0..rows {
            let y = (18.0 / 2.0) * row as f32;
            let y = -y;

            let mut elements= Vec::new();
            for col in 0..cols {
                let x = if row % 2 == 0 {
                    9.9 * 1.5 * col as f32
                } else {
                    (9.9 * 1.5 * col as f32) + 7.5
                };

                positions.push((row, col));
                elements.push(visualizer::component::cell::Cell{
                    position: Vec3::new(x, y, 0.1),
                    entity: None,
                    asigned: false,
                    uuid: Uuid::new_v4(),
                });

            }
            matrix_cells.push(elements);
        }

        Self {
            cols, rows, matrix_cells,
            expanded: false, positions
        }

    }


    pub fn expand_border(&mut self) {
        let new_cols = self.cols + 2;
        let new_rows = self.rows + 2;

        //Generate Cols

        for row in self.matrix_cells.iter_mut() {
            
            row.insert(0, visualizer::component::cell::Cell{
                position: Vec3::new(row.first().unwrap().position.x - 14.85, row.first().unwrap().position.y, 0.1),
                entity: None,
                asigned: false,
                uuid: Uuid::new_v4(),
            });
            
            row.push(visualizer::component::cell::Cell{
                position: Vec3::new(row.last().unwrap().position.x + 14.85, row[0].position.y, 0.1),
                entity: None,
                asigned: false,
                uuid: Uuid::new_v4(),
            });
        }

        

        //Generate rows 

        let mut first_row:Vec<visualizer::component::cell::Cell>= Vec::new();
        let mut last_row:Vec<visualizer::component::cell::Cell>= Vec::new();

        let first_item= self.matrix_cells[0][0].position;
        let last_item= self.matrix_cells.last().unwrap()[0].position;
        let second_x_row= self.matrix_cells[1][0].position;

        let new_x_first= if first_item.x > second_x_row.x{
            first_item.x - 7.5
        }else{
            first_item.x + 7.5
        };

        
        let new_x_last= if first_item.x > second_x_row.x{
            last_item.x + 7.5
        }else{
            last_item.x - 7.5
        };

        let first_y= first_item.y + 9.0;
        let last_y= last_item.y + -9.0;
        for col in 0..new_cols {
            first_row.push( visualizer::component::cell::Cell{
                position: Vec3::new(new_x_first + (14.85 * col as f32), first_y, 0.1),
                entity: None,
                asigned: false,
                uuid: Uuid::new_v4(),
            });
            last_row.push( visualizer::component::cell::Cell{
                position: Vec3::new(new_x_last + (14.85 * col as f32), last_y, 0.1),
                entity: None,
                asigned: false,
                uuid: Uuid::new_v4(),
            });
        }
        self.matrix_cells.insert(0, first_row);
        self.matrix_cells.push(last_row);

        self.cols = new_cols;
        self.rows = new_rows;
        self.expanded= true;


        for (row, cols) in self.matrix_cells.iter().enumerate() {
            for (col, cell) in cols.iter().enumerate() {
                if !cell.asigned {
                    self.positions.push((
                        row, col
                    ));
                }
            }
        }


    }

    pub fn get_free_cell(&mut self)-> &visualizer::component::cell::Cell
    {
        use rand::rng;

        if self.positions.is_empty() {
            self.expand_border();
        }

        let mut rng = rng();
        let index= self.positions.swap_remove(
            rng.random_range(0..self.positions.len())
        );
        self.matrix_cells[index.0][index.1].asigned= true;
        &self.matrix_cells[index.0][index.1]
    }
    

}