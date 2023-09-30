//! Draws a performance graph of the application
//!

use super::watch;
use super::super::vertex_color_shader::Vertex as Vertex;
use super::super::vertex_color_shader::Color as Color;

pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub colors: Vec<Color>,
    pub indices: Vec<u32>,

    watchpoints_size: usize,
}

impl Graph {

    const DURATION_120FPS: instant::Duration = std::time::Duration::from_micros(8333); // 120 fps
    const DURATION_60FPS: instant::Duration = std::time::Duration::from_micros(16666); // 60 fps
    const DURATION_30FPS: instant::Duration= std::time::Duration::from_micros(33333); // 30 fps

    const LEN_PER_MICRO : f32 = Self::LINE_LENGTH as f32 / Self::DURATION_30FPS.as_micros() as f32;

    const OFFSET_X: usize = 10;
    const OFFSET_Y: usize = 10;

    const NR_LINES: usize = 700 - (Self::OFFSET_X + Self::OFFSET_Y);
    const LINE_LENGTH: usize = 200;

    const FPS_LINES: &[Vertex] = &[
        Vertex { position: [Self::OFFSET_X as f32,                         Self::OFFSET_Y as f32 + Self::LEN_PER_MICRO * Self::DURATION_120FPS.as_micros() as f32, 0.0] }, 
        Vertex { position: [Self::OFFSET_X as f32 + Self::NR_LINES as f32, Self::OFFSET_Y as f32 + Self::LEN_PER_MICRO * Self::DURATION_120FPS.as_micros() as f32, 0.0] },
        Vertex { position: [Self::OFFSET_X as f32,                         Self::OFFSET_Y as f32 + Self::LEN_PER_MICRO * Self::DURATION_60FPS.as_micros() as f32, 0.0] }, 
        Vertex { position: [Self::OFFSET_X as f32 + Self::NR_LINES as f32, Self::OFFSET_Y as f32 + Self::LEN_PER_MICRO * Self::DURATION_60FPS.as_micros() as f32, 0.0] }, 
        // Vertex { position: [Self::OFFSET_X as f32,                         Self::OFFSET_Y as f32 + Self::LEN_PER_MICRO * Self::DURATION_30FPS.as_micros() as f32, 0.0] }, 
        // Vertex { position: [Self::OFFSET_X as f32 + Self::NR_LINES as f32, Self::OFFSET_Y as f32 + Self::LEN_PER_MICRO * Self::DURATION_30FPS.as_micros() as f32, 0.0] }, 
    ];


    pub fn new(watchpoints_size: usize) -> Self {
        let line_nr_vertices = watchpoints_size * 2 + 2;

        let mut vertices = vec![Vertex::zero(); line_nr_vertices * Self::NR_LINES + Self::FPS_LINES.len()];
        let mut colors = vec![Color::white(); line_nr_vertices * Self::NR_LINES + Self::FPS_LINES.len()];
        let mut indices = vec![0_u32; line_nr_vertices * Self::NR_LINES + Self::FPS_LINES.len()];

        // vertices
        for i in 0..Self::FPS_LINES.len() {
            let len = vertices.len();
            vertices[i + len - Self::FPS_LINES.len()] = Self::FPS_LINES[i];
        }   

        // colors
        let gradient = colorous::RAINBOW;
  
        for i in 0..colors.len() / line_nr_vertices {

            for j in 0..line_nr_vertices/2 {

                let color = gradient.eval_rational(j, line_nr_vertices/2);

                let r =   color.r as f32 / 255.0;
                let g = color.g as f32 / 255.0;
                let b = color.b as f32 / 255.0;

                colors[i*line_nr_vertices + j*2].color = [r, g, b]; 

                colors[i*line_nr_vertices + j*2+1].color = [r, g, b]; 
            }

            // last two points are gray (show the combined fps)
            let gray = [0.5, 0.5, 0.5];
            colors[i*line_nr_vertices + line_nr_vertices-2].color = gray; 
            colors[i*line_nr_vertices + line_nr_vertices-1].color = gray; 
        }

        // indices
        for (i, item) in indices.iter_mut().enumerate() {
            *item = i as u32;
        }

        Self {
            vertices,
            colors,
            indices,

            watchpoints_size,
        }
    }

    fn create_line(last_update_time: instant::Instant, update_time: instant::Instant, watchpoints: &[watch::Watchpoint]) -> Vec<f32>
    {
        let len = watchpoints.len()*2 + 2;
        let mut line: Vec<f32> = vec![0.0; len];

        // for i in 0..watchpoints.len() {
        for (i, watchpoint) in watchpoints.iter().enumerate() {
            let j = i * 2;

            line[j] =   (watchpoint.start - last_update_time).as_micros() as f32 *  Self::LEN_PER_MICRO;
            line[j+1] = (watchpoint.stop - last_update_time).as_micros() as f32 *  Self::LEN_PER_MICRO;
        }

        line[len-2] = 0.0;
        line[len-1] = (update_time - last_update_time).as_micros() as f32 * Self::LEN_PER_MICRO;

        line
    }

    fn update_vertices(vertices: &mut [Vertex], line: &[f32]) 
    {
        let len = vertices.len();
        let vertices = &mut vertices[0..len - Self::FPS_LINES.len()];

        assert!(!vertices.is_empty());
        assert!(vertices.len() % line.len() == 0);

        vertices.rotate_right(line.len());

        for i in 0..vertices.len() / line.len() {

            for j in 0..line.len() {
                vertices[i*line.len() + j].position[0] = i as f32 + Self::OFFSET_X as f32; 
            }
        }

        for i in 0..line.len() {
            vertices[i].position[1] = line[i] + Self::OFFSET_Y as f32;
        }

    }

}

impl watch::Viewer for Graph {
    fn update(&mut self, last_update_time: instant::Instant, update_time: instant::Instant, watchpoints: &[watch::Watchpoint])
    {
        if watchpoints.len() != self.watchpoints_size {
            return;
        }

        let line = Self::create_line(last_update_time, update_time, watchpoints);
        Self::update_vertices(&mut self.vertices, &line);
    }
}

