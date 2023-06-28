mod gl_bindings;
use gl_bindings::*;

use super::{CheatModule, Cheat};

pub struct Esp {
    cheat: *mut Cheat,
}

impl CheatModule for Esp {
    fn cheat(&self) -> *mut Cheat {
        self.cheat
    }
    unsafe fn run(&self) {
        self.switch_to_2d();
    }
}

impl Esp {
    pub fn new(cheat: *mut Cheat) -> Self {
        Esp { cheat }
    }

    fn switch_to_2d(&self) -> (GLint, GLint) {
        unsafe {
            // save the current state
            gl_bindings::glPushAttrib(GL_ALL_ATTRIB_BITS);

            // save the current matrix
            gl_bindings::glPushMatrix();

            // obtain and set the current viewport (position and dimensions of the window)
            // for the new matrix
            let mut viewport: [GLint; 4] = [0; 4];
            let viewport_ptr = &mut viewport[0] as *mut GLint;
            gl_bindings::glGetIntegerv(GL_VIEWPORT, viewport_ptr);
            gl_bindings::glViewport(0, 0, viewport[2], viewport[3]);

            // go into projection mode
            gl_bindings::glMatrixMode(GL_PROJECTION);

            // loads a blank matrix
            gl_bindings::glLoadIdentity();

            gl_bindings::glOrtho(0.0, viewport[2].into(), viewport[3].into(), 0.0, -1.0, 1.0);

            gl_bindings::glMatrixMode(GL_MODELVIEW);
            gl_bindings::glLoadIdentity();
            gl_bindings::glDisable(GL_DEPTH_TEST);

            (viewport[2], viewport[3])
        }
    }
}
