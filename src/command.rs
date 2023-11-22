use std::{sync::Arc, cell::RefCell};

use glam::Vec3;


trait Command {
    fn execute(&self);
}

