use mlua::{Lua, Table, UserData, Value};
use ratatui::widgets::Widget;

use super::{Rect, Renderable, Text};

// --- List
#[derive(Clone, Default)]
pub struct List {
	area: Rect,

	inner: ratatui::widgets::List<'static>,
}

impl List {
	pub fn install(lua: &Lua, ui: &Table) -> mlua::Result<()> {
		ui.raw_set(
			"List",
			lua.create_function(|_, values: Vec<Value>| {
				let mut items = Vec::with_capacity(values.len());
				for value in values {
					items.push(ratatui::widgets::ListItem::new(Text::try_from(value)?));
				}

				Ok(Self { inner: ratatui::widgets::List::new(items), ..Default::default() })
			})?,
		)
	}
}

impl UserData for List {
	fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
		crate::impl_area_method!(methods);
	}
}

impl Renderable for List {
	fn area(&self) -> ratatui::layout::Rect { *self.area }

	fn render(self: Box<Self>, buf: &mut ratatui::buffer::Buffer) {
		self.inner.render(*self.area, buf);
	}

	fn clone_render(&self, buf: &mut ratatui::buffer::Buffer) { Box::new(self.clone()).render(buf) }
}
