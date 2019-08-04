
pub fn cont_boundary(coord: usize, coord_max: usize, offset: i8) -> usize{
    if (coord as i8) + offset <0  {return coord_max -1};
    if (coord as i8) + offset == (coord_max as i8)  {return 0};
    return coord + (offset as usize);
}