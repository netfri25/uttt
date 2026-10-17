use macroquad::prelude::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame(pub Rect);

impl Frame {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self(Rect { x, y, w, h })
    }

    /// splits the frame into multiple frames in the horizontal axis
    /// 2 splits -> 3 frames
    pub fn split_w(self, splits: u32) -> impl Iterator<Item = Self> {
        let w = self.w / (splits + 1) as f32;
        (0..=splits).map(move |i| Self::new(self.x + w * i as f32, self.y, w, self.h))
    }

    /// splits the frame into multiple frames in the vertical axis
    /// 2 splits -> 3 frames
    pub fn split_h(self, splits: u32) -> impl Iterator<Item = Self> {
        let h = self.h / (splits + 1) as f32;
        (0..=splits).map(move |i| Self::new(self.x, self.y + h * i as f32, self.w, h))
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

    /// shrink the frame to its center in the horizontal axis
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

    /// shrink the frame to its center in the vertical axis
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

    /// shrink the frame to its center
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
        Self(value)
    }
}
