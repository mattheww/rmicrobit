use microbit_blinkenlights::Render;
use microbit_blinkenlights::image::GreyscaleImage;
use microbit_blinkenlights::scrolling::{Animate, ScrollingImages};
use microbit_blinkenlights::scrolling_text;

pub trait Animator {

    type Image: Render;

    fn next(&mut self);
    fn get_image(&self) -> Self::Image;

}

pub trait RefAnimator {

    type Image: Render;

    fn next(&mut self);
    fn get_image_ref(&self) -> &Self::Image;

}


struct Ticker {
    slowdown: usize,
    subtick: usize,
}

impl Ticker {

    fn reset(&mut self, slowdown: usize) {
        self.slowdown = slowdown;
        self.subtick = 0;
    }

    fn tick(&mut self) -> bool {
        self.subtick += 1;
        if self.subtick == self.slowdown {
            self.subtick = 0;
            true
        } else {
            false
        }
    }
}

impl Default for Ticker {

    fn default() -> Ticker {
        Ticker {slowdown: 1, subtick: 0}
    }

}


///////////

pub struct FunctionalAnimation {
    pub length: usize,
    pub render: fn (usize) -> GreyscaleImage,
}

impl FunctionalAnimation {
    const BLANK: FunctionalAnimation = FunctionalAnimation {
        length: 1,
        render: {
            fn blank(_index: usize) -> GreyscaleImage {GreyscaleImage::blank()};
            blank
        },
    };
}

pub struct FunctionalAnimator {
    frame_index: usize,
    animation: &'static FunctionalAnimation,
}

impl FunctionalAnimator {

    pub fn reset(&mut self, animation: &'static FunctionalAnimation) {
        self.frame_index = 0;
        self.animation = animation;
    }

}

impl Default for FunctionalAnimator {

    fn default() -> FunctionalAnimator {
        FunctionalAnimator {
            frame_index: 0,
            animation: &FunctionalAnimation::BLANK,
        }
    }

}

impl Animator for FunctionalAnimator {

    type Image = GreyscaleImage;

    fn next(&mut self) {
        self.frame_index += 1;
        if self.frame_index == self.animation.length {
            self.frame_index = 0;
        }
    }

    fn get_image(&self) -> Self::Image {
        (self.animation.render)(self.frame_index)
    }

}

///////////

#[derive(Default)]
pub struct ScrollingImageAnimator {
    ticker: Ticker,
    scroller: ScrollingImages<&'static GreyscaleImage>,
}

impl ScrollingImageAnimator {

    pub fn reset(&mut self, images: &'static [&'static GreyscaleImage], slowdown: usize) {
        self.ticker.reset(slowdown);
        self.scroller.set_images(images);
    }

}

impl RefAnimator for ScrollingImageAnimator {

    type Image = ScrollingImages<&'static GreyscaleImage>;

    fn next(&mut self) {
        if self.ticker.tick() {
            self.scroller.tick();
        }
    }

    fn get_image_ref(&self) -> &Self::Image {
        &self.scroller
    }

}


///////////

#[derive(Default)]
pub struct ScrollingStaticTextAnimator {
    ticker: Ticker,
    scroller: scrolling_text::ScrollingStaticText,
}

impl ScrollingStaticTextAnimator {

    pub fn reset(&mut self, message: &'static [u8], slowdown: usize) {
        self.ticker.reset(slowdown);
        self.scroller.set_message(message);
    }

}

impl RefAnimator for ScrollingStaticTextAnimator {

    type Image = scrolling_text::ScrollingStaticText;

    fn next(&mut self) {
        if self.ticker.tick() {
            self.scroller.tick();
        }
    }

    fn get_image_ref(&self) -> &Self::Image {
        &self.scroller
    }

}


///////////

#[derive(Default)]
pub struct ScrollingBufferedTextAnimator {
    ticker: Ticker,
    scroller: scrolling_text::ScrollingBufferedText,
}

impl ScrollingBufferedTextAnimator {

    pub fn reset(&mut self, message: &[u8], slowdown: usize) {
        self.ticker.reset(slowdown);
        self.scroller.set_message(message);
    }

}

impl RefAnimator for ScrollingBufferedTextAnimator {

    type Image = scrolling_text::ScrollingBufferedText;

    fn next(&mut self) {
        if self.ticker.tick() {
            self.scroller.tick();
        }
    }

    fn get_image_ref(&self) -> &Self::Image {
        &self.scroller
    }

}

