use macroquad::prelude::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame(pub Rect);

impl Frame {
    pub fn new(rect: Rect) -> Self {
        Self(rect)
    }

    /// convert rectangle into a centered square
    pub fn to_square(self) -> Self {
        let size = self.h.min(self.w);

        Self(Rect {
            w: size,
            h: size,
            x: self.x + (self.w - size) / 2.,
            y: self.y + (self.h - size) / 2.,
        })
    }

    #[inline(always)]
    pub fn shrink_w(self, percent: f32) -> Self {
        let percent = percent.clamp(0., 1.);
        let diff_w = self.w * percent;
        Self(Rect {
            w: self.w - diff_w,
            h: self.h,
            x: self.x + diff_w / 2.,
            y: self.y,
        })
    }

    #[inline(always)]
    pub fn shrink_h(self, percent: f32) -> Self {
        let percent = percent.clamp(0., 1.);
        let diff_h = self.h * percent;
        Self(Rect {
            w: self.w,
            h: self.h - diff_h,
            x: self.x,
            y: self.y + diff_h / 2.,
        })
    }

    #[inline]
    pub fn shrink(self, percent: f32) -> Self {
        let percent = percent.clamp(0., 1.);
        self.shrink_w(percent).shrink_h(percent)
    }
}

impl std::ops::DerefMut for Frame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Frame {
    type Target = Rect;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Rect> for Frame {
    fn from(value: Rect) -> Self {
        Self::new(value)
    }
}
