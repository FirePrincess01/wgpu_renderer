//! Draws a performance graph of the application
//!

use super::super::vertex_color_shader::Color;
use super::super::vertex_color_shader::Vertex;
use super::watch;

struct Geometry {
    len_per_micro: f32,

    offset_x: usize,
    offset_y: usize,

    width: usize,
    height: usize,
    nr_lines: usize,
    _line_length: usize,

    fps_lines: [Vertex; 6],
}

pub struct Graph<const SIZE: usize> {
    pub vertices: Vec<Vertex>,
    pub colors: Vec<Color>,
    pub indices: Vec<u32>,

    watch_points_size: usize,
    color_gradient: colorous::Gradient,

    geometry: Geometry,

}

impl<const SIZE: usize> Graph<SIZE> {
    const DURATION_120FPS: instant::Duration = std::time::Duration::from_micros(8333); // 120 fps
    const DURATION_60FPS: instant::Duration = std::time::Duration::from_micros(16666); // 60 fps
    const DURATION_30FPS: instant::Duration = std::time::Duration::from_micros(33333); // 30 fps

    fn create_geometry(scale_factor: f32) -> Geometry {
        
        const OFFSET_X: usize = 10;
        const OFFSET_Y: usize = 10;

        let width: usize = (700_f32 * scale_factor) as usize;
        let height: usize = (210_f32 * scale_factor) as usize;
        let nr_lines: usize = width - OFFSET_X;
        let line_length: usize = height - OFFSET_Y;
    

        let len_per_micro: f32 = line_length as f32 / Self::DURATION_30FPS.as_micros() as f32;



        let fps_lines: [Vertex; 6] = [
            Vertex {
                position: [OFFSET_X as f32, OFFSET_Y as f32, 0.0],
            },
            Vertex {
                position: [
                    OFFSET_X as f32 + nr_lines as f32,
                    OFFSET_Y as f32,
                    0.0,
                ],
            },
            Vertex {
                position: [
                    OFFSET_X as f32,
                    OFFSET_Y as f32
                        + len_per_micro * Self::DURATION_120FPS.as_micros() as f32,
                    0.0,
                ],
            },
            Vertex {
                position: [
                    OFFSET_X as f32 + nr_lines as f32,
                    OFFSET_Y as f32
                        + len_per_micro * Self::DURATION_120FPS.as_micros() as f32,
                    0.0,
                ],
            },
            Vertex {
                position: [
                    OFFSET_X as f32,
                    OFFSET_Y as f32
                        + len_per_micro * Self::DURATION_60FPS.as_micros() as f32,
                    0.0,
                ],
            },
            Vertex {
                position: [
                    OFFSET_X as f32 + nr_lines as f32,
                    OFFSET_Y as f32
                        + len_per_micro * Self::DURATION_60FPS.as_micros() as f32,
                    0.0,
                ],
            },
            // Vertex {
            //     position: [
            //         OFFSET_X as f32,
            //         OFFSET_Y as f32
            //             + len_per_micro * Self::DURATION_30FPS.as_micros() as f32,
            //         0.0,
            //     ],
            // },
            // Vertex {
            //     position: [
            //         OFFSET_X as f32 + nr_lines as f32,
            //         OFFSET_Y as f32
            //             + len_per_micro * Self::DURATION_30FPS.as_micros() as f32,
            //         0.0,
            //     ],
            // },
        ];

        Geometry {
            len_per_micro,
            offset_x: OFFSET_X,
            offset_y: OFFSET_Y,
            width,
            height,
            nr_lines,
            _line_length: line_length,
            fps_lines,
        }
    }

    pub fn color_gradient_vec(
        color_gradient: &colorous::Gradient,
        watch_points_size: usize,
    ) -> Vec<cgmath::Vector3<f32>> {
        let gradient = color_gradient;

        let mut colors = Vec::with_capacity(watch_points_size);

        for i in 0..(watch_points_size) {
            let color = gradient.eval_rational(i, watch_points_size);
            colors.push(cgmath::Vector3 {
                x: color.r as f32 / 255.0,
                y: color.g as f32 / 255.0,
                z: color.b as f32 / 255.0,
            });
        }

        colors
    }

    pub fn color_gradient(&self) -> Vec<cgmath::Vector3<f32>> {
        Self::color_gradient_vec(&self.color_gradient, self.watch_points_size)
    }

    pub fn new(color_gradient: colorous::Gradient, scale_factor: f32) -> Self {
        let watch_points_size = SIZE;
        let line_nr_vertices = watch_points_size * 2 + 2;

        let geometry = Self::create_geometry(scale_factor);

        let mut vertices =
            vec![Vertex::zero(); line_nr_vertices * geometry.nr_lines + geometry.fps_lines.len()];
        let mut colors =
            vec![Color::white(); line_nr_vertices * geometry.nr_lines + geometry.fps_lines.len()];
        let mut indices = vec![0_u32; line_nr_vertices * geometry.nr_lines + geometry.fps_lines.len()];

        // vertices
        vertices[..geometry.fps_lines.len()].copy_from_slice(&geometry.fps_lines);

        // colors
        let color_gradient_vec = Self::color_gradient_vec(&color_gradient, watch_points_size);

        for i in 0..colors.len() / line_nr_vertices {
            for j in 0..watch_points_size {
                let color = color_gradient_vec[j];

                colors[geometry.fps_lines.len() + i * line_nr_vertices + j * 2].color = color.into();
                colors[geometry.fps_lines.len() + i * line_nr_vertices + j * 2 + 1].color =
                    color.into();
            }

            // last two points are gray (show the combined fps)
            let gray = [0.5, 0.5, 0.5];
            colors[geometry.fps_lines.len() + i * line_nr_vertices + line_nr_vertices - 2].color =
                gray;
            colors[geometry.fps_lines.len()+ i * line_nr_vertices + line_nr_vertices - 1].color =
                gray;
        }

        // indices
        for (i, item) in indices.iter_mut().enumerate() {
            *item = i as u32;
        }

        Self {
            vertices,
            colors,
            indices,

            watch_points_size,
            color_gradient,

            geometry,
        }
    }

    fn create_line(
        geometry: &Geometry,
        last_update_time: instant::Instant,
        update_time: instant::Instant,
        watch_points: &[watch::WatchPoint],
    ) -> Vec<f32> {
        let len = watch_points.len() * 2 + 2;
        let mut line: Vec<f32> = vec![0.0; len];

        for (i, watch_point) in watch_points.iter().enumerate() {
            let j = i * 2;

            let micros_start = if watch_point.start > last_update_time {
                (watch_point.start - last_update_time).as_micros() as f32 * geometry.len_per_micro
            } else {
                0.0
            };

            let micros_stop = if watch_point.start > last_update_time {
                (watch_point.stop - last_update_time).as_micros() as f32 * geometry.len_per_micro
            } else {
                0.0
            };

            line[j] = micros_start;
            line[j + 1] = micros_stop;
        }

        let micros = if update_time > last_update_time {
            (update_time - last_update_time).as_micros() as f32 * geometry.len_per_micro
        } else {
            0.0
        };

        line[len - 2] = 0.0;
        line[len - 1] = micros;

        line
    }

    fn update_vertices(geometry: &Geometry, vertices: &mut [Vertex], line: &[f32]) {
        assert!(!vertices.is_empty());
        assert!(vertices.len().is_multiple_of(line.len()));

        vertices.rotate_right(line.len());

        for i in 0..vertices.len() / line.len() {
            for j in 0..line.len() {
                vertices[i * line.len() + j].position[0] = i as f32 + geometry.offset_x as f32;
            }
        }

        for i in 0..line.len() {
            vertices[i].position[1] = line[i] + geometry.offset_y as f32;
        }
    }

    pub fn get_height(&self) -> usize {
        self.geometry.height
    }

    pub fn get_height_30fps(&self) -> f32 {
        if self.geometry.fps_lines.len() >= 8 {
            // self.geometry.fps_lines[6].position[1]
            0.0
        } else {
            0.0
        }
    }

    pub fn get_height_60fps(&self) -> f32 {
        self.geometry.fps_lines[4].position[1]
    }

    pub fn get_height_120fps(&self) -> f32 {
        self.geometry.fps_lines[2].position[1]
    }

    pub fn get_width(&self) -> usize {
        self.geometry.width
    }

    pub fn get_nr_lines(&self) -> usize {
        self.geometry.nr_lines
    }

    pub fn update_from_viewer_data(&mut self, data: &watch::WatchViewerData<SIZE>) {
        let last_update_time = data.last_update_time;
        let update_time = data.update_time;
        let watch_points = &data.watch_points;

        if watch_points.len() != self.watch_points_size {
            return;
        }

        let fps_lines_les = self.geometry.fps_lines.len();
        let line = Self::create_line(&self.geometry, last_update_time, update_time, watch_points);
        Self::update_vertices(&self.geometry, &mut self.vertices[fps_lines_les..], &line);
    }
}
