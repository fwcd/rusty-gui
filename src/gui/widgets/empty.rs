use super::widget::Widget;
use super::bounds::WidgetBounds;
use super::base::WidgetBase;
use utils::size::Size;
use gui::core::graphics::Graphics;
use gui::themes::theme::Theme;

pub struct EmptyWidget {
	base: WidgetBase
}

impl EmptyWidget {
	pub fn new() -> EmptyWidget { EmptyWidget { base: WidgetBase::empty() } }
}

impl Widget for EmptyWidget {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {}
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size { Size::of(0, 0) }
	
	fn bounds(&self) -> &WidgetBounds { &self.base.bounds }
	
	fn set_bounds(&mut self, bounds: WidgetBounds) { self.base.bounds = bounds }
}