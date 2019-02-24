use numtoa::NumToA;

use microbit_blinkenlights::{Render, MAX_BRIGHTNESS, font};
use microbit_blinkenlights::image::{GreyscaleImage, BitImage};

use crate::animation::{
    Animator, RefAnimator,
    FunctionalAnimation, FunctionalAnimator,
    ScrollingImageAnimator, ScrollingStaticTextAnimator,
    ScrollingBufferedTextAnimator,
};

const HEART: BitImage = BitImage::new(&[
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
]);

const GREY_HEART: GreyscaleImage = GreyscaleImage::new(&[
    [0, 9, 0, 9, 0],
    [9, 5, 9, 5, 9],
    [9, 5, 5, 5, 9],
    [0, 9, 5, 9, 0],
    [0, 0, 9, 0, 0],
]);

const SCALE: GreyscaleImage = GreyscaleImage::new(&[
    [0, 0, 1, 1, 0],
    [3, 3, 2, 2, 0],
    [4, 4, 5, 5, 0],
    [7, 7, 6, 6, 0],
    [8, 8, 9, 9, 0],
]);

const ONE_TWO: GreyscaleImage = GreyscaleImage::new(&[
    [2, 2, 2, 2, 1],
    [2, 2, 2, 1, 1],
    [2, 2, 1, 1, 1],
    [2, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
]);

fn chequer_image(brightness: u8) -> GreyscaleImage {
    let b = brightness;
    let w = MAX_BRIGHTNESS as u8 - brightness;
    GreyscaleImage::new(&[
        [b, w, b, w, b],
        [w, b, w, b, w],
        [b, w, b, w, b],
        [w, b, w, b, w],
        [b, w, b, w, b],
    ])
}

fn stripe_image(brightness: u8) -> GreyscaleImage {
    let even = brightness;
    let odd = MAX_BRIGHTNESS as u8 - brightness;
    GreyscaleImage::new(&[
        [even; 5],
        [odd; 5],
        [even; 5],
        [odd; 5],
        [even; 5],
    ])
}

fn heart_image(brightness: u8) -> GreyscaleImage {
    let b = brightness;
    GreyscaleImage::new(&[
        [0, 7, 0, 7, 0],
        [7, b, 7, b, 7],
        [7, b, b, b, 7],
        [0, 7, b, 7, 0],
        [0, 0, 7, 0, 0],
    ])
}

fn render_chequers(frame_index: usize) -> GreyscaleImage {
    let grey_level = match frame_index {
        0..=9 => frame_index,
        10..=17 => (18 - frame_index),
        _ => unreachable!()
    };
    chequer_image(grey_level as u8)
}

fn render_stripes(frame_index: usize) -> GreyscaleImage {
    let grey_level = match frame_index {
        0..=9 => frame_index,
        10..=17 => (18 - frame_index),
        _ => unreachable!()
    };
    stripe_image(grey_level as u8)
}

fn render_heart(frame_index: usize) -> GreyscaleImage {
    let grey_level = match frame_index {
        0..=8 => 9-frame_index,
        9..=12 => 0,
        _ => unreachable!()
    };
    heart_image(grey_level as u8)
}


enum Scene {
    Static {
        image: &'static dyn Render,
    },
    Animation {
        animation: &'static FunctionalAnimation,
    },
    ScrollImages {
        images: &'static [&'static GreyscaleImage],
        slowdown: usize,
    },
    ScrollText {
        message: &'static [u8],
        slowdown: usize,
    },
    Counter,
    Font,
}

const SCENE_COUNT: usize = 12;

const SCENES: [Scene; SCENE_COUNT] = [
    Scene::Static {image: &HEART},
    Scene::Static {image: &GREY_HEART},
    Scene::Static {image: &SCALE},
    Scene::Static {image: &ONE_TWO},
    Scene::Animation {
        animation: &FunctionalAnimation {length: 12, render: render_heart}
    },
    Scene::Animation {
        animation: &FunctionalAnimation {length: 18, render: render_chequers}
    },
    Scene::Animation {
        animation: &FunctionalAnimation {length: 18, render: render_stripes}
    },
    Scene::ScrollText {
        message: b"Hello, world!",
        slowdown: 2,
    },
    Scene::ScrollImages {
        images: &[&GREY_HEART, &SCALE, &GREY_HEART, &ONE_TWO, &GREY_HEART],
        slowdown: 2},
    Scene::ScrollText {
        message: b"Try button B",
        slowdown: 1
    },
    Scene::Counter,
    Scene::Font,
];

pub fn initial_frame() -> &'static impl Render {
    &HEART
}

pub struct Demo {
    scene_index: usize,
    animator: FunctionalAnimator,
    si_animator: ScrollingImageAnimator,
    sst_animator: ScrollingStaticTextAnimator,
    sbt_animator: ScrollingBufferedTextAnimator,
    counter_index: usize,
    font_index: u8,
}

impl Demo {

    pub fn new() -> Demo {
        Demo{scene_index:0,
             animator: Default::default(),
             si_animator: Default::default(),
             sst_animator: Default::default(),
             sbt_animator: Default::default(),
             counter_index: 0,
             font_index: 0,
        }
    }

    fn current_scene(&self) -> &'static Scene {
        &SCENES[self.scene_index]
    }

    fn set_counter(&mut self, value: usize) {
        self.counter_index = value;
        let slowdown: usize = 2;
        let mut buffer = [0; 12];
        self.sbt_animator.reset(
            self.counter_index.numtoa(10, &mut buffer),
            slowdown);
    }

    pub fn next_state(&mut self) {
        self.scene_index += 1;
        if self.scene_index == SCENE_COUNT {self.scene_index = 0}
        match *self.current_scene() {
            Scene::Animation {animation} => {
                self.animator.reset(animation);
            },
            Scene::ScrollImages {images, slowdown} => {
                self.si_animator.reset(images, slowdown);
            },
            Scene::ScrollText {message, slowdown} => {
                self.sst_animator.reset(message, slowdown);
            },
            Scene::Counter => {
                self.set_counter(1);
            },
            Scene::Font => {
                self.font_index = 1;
            },
            _ => ()
        }
    }

    pub fn next_state_or_modify_current_state(&mut self) {
        match *self.current_scene() {
            Scene::Counter => {
                self.set_counter(self.counter_index+1);
            }
            Scene::Font => {
                self.font_index += 1;
                if self.font_index == font::PRINTABLE_COUNT as u8 {
                    self.font_index = 1;
                };
            }
            _ => {
                self.next_state();
            }
        };
    }

    pub fn is_static(&self) -> bool {
        match *self.current_scene() {
            Scene::Animation {..} => false,
            Scene::ScrollImages {..} => false,
            Scene::ScrollText {..} => false,
            Scene::Counter => false,
            _ => true,
        }
    }

    pub fn is_animating(&self) -> bool {
        match *self.current_scene() {
            Scene::Animation {..} => true,
            _ => false,
        }
    }

    pub fn is_scrolling(&self) -> bool {
        match *self.current_scene() {
            Scene::ScrollImages {..} => true,
            Scene::ScrollText {..} => true,
            Scene::Counter => true,
            _ => false,
        }
    }

    pub fn current_image(&mut self) -> &dyn Render {
        match *self.current_scene() {
            Scene::Static {image} => image,
            Scene::Font =>
                font::character(font::PRINTABLE_START as u8 + self.font_index),
            _ => panic!("not static"),
        }
    }

    pub fn next_animation_frame(&mut self) -> impl Render {
        match *self.current_scene() {
            Scene::Animation {..} => {
                let image = self.animator.get_image();
                self.animator.next();
                image
            }
            _ => panic!("not animating"),
        }
    }

    pub fn next_scrolling_frame(&mut self) -> &dyn Render {
        match *self.current_scene() {
            Scene::ScrollImages {..} => {
                self.si_animator.next();
                self.si_animator.get_image_ref()
            },
            Scene::ScrollText {..} => {
                self.sst_animator.next();
                self.sst_animator.get_image_ref()
            },
            Scene::Counter => {
                self.sbt_animator.next();
                self.sbt_animator.get_image_ref()
            },
            _ => panic!("not scrolling"),
        }
    }

}
