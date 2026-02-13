use crate::chip8::parameters::DISPLAY_SIZE;

#[derive(PartialEq, Copy, Clone)]
pub struct Display{
    pub plane_1: [bool; DISPLAY_SIZE],
    pub plane_2: [bool; DISPLAY_SIZE],
    pub selected_plane: u8,
}
impl Display{
    pub fn new() -> Display{
        Display{
            plane_1: [false; DISPLAY_SIZE],
            plane_2: [false; DISPLAY_SIZE],
            selected_plane: 1,
        }
    }
    pub fn get_selected_planes(&mut self) -> Vec<&mut [bool; DISPLAY_SIZE]>{
        match self.selected_plane{
            1 => vec![&mut self.plane_1],
            2 => vec![&mut self.plane_2],
            3 => vec![&mut self.plane_1, &mut self.plane_2],
            _ => vec![]
        }
    }
    pub fn execute_scroll(&mut self, scroll_function: fn(display: &mut [bool;DISPLAY_SIZE], n: usize), n: usize){
        for plane in self.get_selected_planes(){
            scroll_function(plane,n);
        }
    }
}