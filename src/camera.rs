use cgmath::{SquareMatrix, InnerSpace, Vector3, Zero};
use winit::{event::*, dpi::PhysicalSize};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub(crate) struct Camera {
    pub pos: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub resolution: PhysicalSize<u32>
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.pos, self.pos + Vector3::new(self.yaw.cos(), self.pitch.tan(), self.yaw.sin()), self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

pub(crate) struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    mouse_delta: (f64, f64),
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn process_events(&mut self, device_event: Option<&DeviceEvent>, window_event: Option<&WindowEvent>) -> bool {
        device_event.and_then(|event| Some({
            match event {
                DeviceEvent::Key(KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                }) => {
                    let is_pressed = *state == ElementState::Pressed;
                    match key {
                        VirtualKeyCode::Space => {
                            self.is_up_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::LShift => {
                            self.is_down_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::W => {
                            self.is_forward_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::A => {
                            self.is_left_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::S => {
                            self.is_backward_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::D => {
                            self.is_right_pressed = is_pressed;
                            true
                        }
                        _ => false,
                    }
                }
                DeviceEvent::MouseMotion { delta } => {
                    self.mouse_delta = *delta;
                    true
                }
                _ => false
            };
        }));
        window_event.and_then(|event| Some({ 
            match event {
                _ => false
            }
        }));
        false
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        let forward = Vector3::new(camera.yaw.cos(), camera.pitch, camera.yaw.sin());
        let mut movement = Vector3::zero();

        // Forward backward movement
        if self.is_forward_pressed { movement += forward; }
        if self.is_backward_pressed { movement -= forward; }

        // Left right movement
        let right = forward.cross(camera.up);
        if self.is_left_pressed { movement -= right; }
        if self.is_right_pressed { movement += right; }

        // Up down movement
        if self.is_up_pressed { movement.y += 1.0; }
        if self.is_down_pressed { movement.y -= 1.0; }

        // Normalize is NAN if zero
        if movement.x + movement.y + movement.z != 0.0 {
            camera.pos += movement.normalize() * self.speed;
        } else {
            camera.pos += movement * self.speed;
        }

        // Mouse movement
        camera.yaw += (self.mouse_delta.0 / 100.0) as f32;
        let pitch_delta = (self.mouse_delta.1 / 100.0) as f32;
        if (pitch_delta < 0.0 && camera.pitch < 1.5) || (pitch_delta > 0.0 && camera.pitch > -1.5) {
            camera.pitch -= pitch_delta;
        }
        self.mouse_delta = (0.0, 0.0)
    }
}