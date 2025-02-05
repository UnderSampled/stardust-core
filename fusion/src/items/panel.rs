use super::Item;
use crate::{
	client::Client,
	drawable::ModelPart,
	node::{HandledNodeType, Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper,
};
use flagset::{flags, FlagSet};
use mint::Vector2;
use parking_lot::Mutex;
use serde::{
	de::{Deserializer, Error, SeqAccess, Visitor},
	ser::Serializer,
	Deserialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};
use stardust_xr::schemas::flex::deserialize;
use std::{ops::Deref, sync::Arc};

/// Handler for the `panel` item.
#[allow(unused_variables)]
pub trait PanelItemHandler: Send + Sync {
	/// The toplevel surface's state has just been updated to `state`.
	fn commit_toplevel(&mut self, state: Option<ToplevelInfo>);

	/// The toplevel surface recommends that you set the state to this.
	///
	/// You don't have to though.
	fn recommend_toplevel_state(&mut self, state: RequestedState) {}

	/// The toplevel surface requests that you show a menu to control the window, like if you right clicked the client side decorations.
	fn show_window_menu(&mut self) {}

	/// The cursor is being changed.
	///
	/// The cursor's material will automatically update, you just need to hide/show the cursor and account for the new size/hotspot.
	fn set_cursor(&mut self, info: Option<CursorInfo>) {}

	/// A new popup was created. Popups are short-lived overlay surfaces like for context menus or modals.
	fn new_popup(&mut self, uid: &str, data: Option<PopupInfo>) {}
	/// The popup's positioner has changed and you need to update the UI.
	fn reposition_popup(&mut self, uid: &str, data: Option<PositionerData>) {}
	/// This popup was destroyed.
	fn drop_popup(&mut self, uid: &str) {}

	/// The given surface is the only one that should recieve pointer input. If None, any surface in this panel item can receive pointer input.
	fn grab_pointer_focus(&mut self, focus: Option<SurfaceID>) {}
	/// The given surface is the only one that should recieve keyboard input. If None, any surface in this panel item can receive keyboard input.
	fn grab_keyboard_focus(&mut self, focus: Option<SurfaceID>) {}
}

/// An updated cursor.
#[derive(Debug, Clone, Deserialize)]
pub struct CursorInfo {
	/// Size of the cursor in pixels.
	pub size: Vector2<u32>,
	/// Hotspot position in pixels. This is the point relative to the top left where the cursor matches the 2D pointer.
	pub hotspot: Vector2<i32>,
}
/// The origin and size of the surface's "solid" part.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Geometry {
	pub origin: Vector2<i32>,
	pub size: Vector2<u32>,
}
/// The state of the panel item's toplevel.
#[derive(Debug, Clone, Deserialize)]
pub struct ToplevelInfo {
	/// The UID of the panel item of the parent of this toplevel, if it exists
	pub parent: Option<String>,
	/// Equivalent to the window title
	pub title: Option<String>,
	/// Application identifier, see <https://standards.freedesktop.org/desktop-entry-spec/>
	pub app_id: Option<String>,
	/// Current size in pixels
	pub size: Vector2<u32>,
	/// Recommended minimum size in pixels
	pub min_size: Option<Vector2<u32>>,
	/// Recommended maximum size in pixels
	pub max_size: Option<Vector2<u32>>,
	/// Surface geometry
	pub geometry: Geometry,
	/// Array of states
	pub states: Vec<State>,
}

/// Data on positioning a popup
#[derive(Debug, Clone, Deserialize)]
pub struct PopupInfo {
	pub uid: String,
	pub parent: SurfaceID,
	pub positioner_data: PositionerData,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Deserialize_repr, PartialEq, Eq)]
pub enum Alignment {
	TopLeft = 0,
	Top = 1,
	TopRight = 2,
	Left = 3,
	Center = 4,
	Right = 5,
	BottomLeft = 6,
	Bottom = 7,
	BottomRight = 8,
}

flags! {
	pub enum ConstraintAdjustment: u32 {
		None = 0,
		SlideX = 1,
		SlideY = 2,
		FlipX = 4,
		FlipY = 8,
		ResizeX = 16,
		ResizeY = 32,
	}
}

/// Data on positioning a popup
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct PositionerData {
	/// The size of the surface in px.
	pub size: Vector2<u32>,

	/// The position of the rectangle that the positioned surface will be anchored to.
	pub anchor_rect_pos: Vector2<i32>,

	/// The size of the rectangle that the positioned surface will be anchored to.
	pub anchor_rect_size: Vector2<u32>,

	/// Specifies the point on the surface that will be anchored to the anchor rectangle.
	pub anchor: Alignment,

	/// Defines how the surface should be positioned relative to the anchor rectangle.
	pub gravity: Alignment,

	/// Specifies the adjustment behavior of the positioner when the surface would fall outside the screen.
	pub constraint_adjustment: FlagSet<ConstraintAdjustment>,

	/// The offset of the surface relative to the anchor rectangle.
	pub offset: Vector2<i32>,

	/// Whether the positioner reacts to changes in the geometry of the anchor rectangle.
	pub reactive: bool,
}
impl PositionerData {
	fn anchor_has_edge(&self, edge: Alignment) -> bool {
		match edge {
			Alignment::Top => {
				self.anchor == Alignment::Top
					|| self.anchor == Alignment::TopLeft
					|| self.anchor == Alignment::TopRight
			}
			Alignment::Bottom => {
				self.anchor == Alignment::Bottom
					|| self.anchor == Alignment::BottomLeft
					|| self.anchor == Alignment::BottomRight
			}
			Alignment::Left => {
				self.anchor == Alignment::Left
					|| self.anchor == Alignment::TopLeft
					|| self.anchor == Alignment::BottomLeft
			}
			Alignment::Right => {
				self.anchor == Alignment::Right
					|| self.anchor == Alignment::TopRight
					|| self.anchor == Alignment::BottomRight
			}
			_ => unreachable!(),
		}
	}

	fn gravity_has_edge(&self, edge: Alignment) -> bool {
		match edge {
			Alignment::Top => {
				self.gravity == Alignment::Top
					|| self.gravity == Alignment::TopLeft
					|| self.gravity == Alignment::TopRight
			}
			Alignment::Bottom => {
				self.gravity == Alignment::Bottom
					|| self.gravity == Alignment::BottomLeft
					|| self.gravity == Alignment::BottomRight
			}
			Alignment::Left => {
				self.gravity == Alignment::Left
					|| self.gravity == Alignment::TopLeft
					|| self.gravity == Alignment::BottomLeft
			}
			Alignment::Right => {
				self.gravity == Alignment::Right
					|| self.gravity == Alignment::TopRight
					|| self.gravity == Alignment::BottomRight
			}
			_ => unreachable!(),
		}
	}

	pub fn get_pos(&self) -> Vector2<i32> {
		let mut pos = self.offset;

		if self.anchor_has_edge(Alignment::Top) {
			pos.y += self.anchor_rect_pos.y;
		} else if self.anchor_has_edge(Alignment::Bottom) {
			pos.y += self.anchor_rect_pos.y + self.anchor_rect_size.y as i32;
		} else {
			pos.y += self.anchor_rect_pos.y + self.anchor_rect_size.y as i32 / 2;
		}

		if self.anchor_has_edge(Alignment::Left) {
			pos.x += self.anchor_rect_pos.x;
		} else if self.anchor_has_edge(Alignment::Right) {
			pos.x += self.anchor_rect_pos.x + self.anchor_rect_size.x as i32;
		} else {
			pos.x += self.anchor_rect_pos.x + self.anchor_rect_size.x as i32 / 2;
		}

		if self.gravity_has_edge(Alignment::Top) {
			pos.y -= self.size.y as i32;
		} else if !self.gravity_has_edge(Alignment::Bottom) {
			pos.y -= self.size.y as i32 / 2;
		}

		if self.gravity_has_edge(Alignment::Left) {
			pos.x -= self.size.x as i32;
		} else if !self.gravity_has_edge(Alignment::Right) {
			pos.x -= self.size.x as i32 / 2;
		}

		pos
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum RequestedState {
	Maximize(bool),
	Fullscreen(bool),
	Minimize,
	Move,
	Resize(Edge),
}
#[derive(Debug, Clone, Deserialize_repr)]
#[repr(u32)]
pub enum Edge {
	None = 0,
	Top = 1,
	Bottom = 2,
	Left = 4,
	TopLeft = 5,
	BottomLeft = 6,
	Right = 8,
	TopRight = 9,
	BottomRight = 10,
}

/// The states the toplevel can be in.
#[repr(u32)]
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
pub enum State {
	/// The surface is maximized.
	///
	/// The window geometry specified in the configure event must be obeyed by the client.
	/// The client should draw without shadow or other decoration outside of the window geometry.
	Maximized = 1,
	/// The surface is fullscreen.
	///
	/// The window geometry specified in the configure event is a maximum; the client cannot resize beyond it.
	/// For a surface to cover the whole fullscreened area, the geometry dimensions must be obeyed by the client.
	Fullscreen = 2,
	/// The surface is being resized.
	///
	/// The window geometry specified in the configure event is a maximum; the client cannot resize beyond it.
	/// Clients that have aspect ratio or cell sizing configuration can use a smaller size, however.
	Resizing = 3,
	/// Client window decorations should be painted as if the window is active.
	///
	/// This does not mean that the window actually has keyboard or pointer focus.
	Activated = 4,
	/// The window is currently in a tiled layout and the left edge is considered to be adjacent to another part of the tiling grid.
	TiledLeft = 5,
	/// The window is currently in a tiled layout and the right edge is considered to be adjacent to another part of the tiling grid.
	TiledRight = 6,
	/// The window is currently in a tiled layout and the top edge is considered to be adjacent to another part of the tiling grid.
	TiledTop = 7,
	/// The window is currently in a tiled layout and the bottom edge is considered to be adjacent to another part of the tiling grid.
	TiledBottom = 8,
}
/// A capability the panel item UI has.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
pub enum Capability {
	/// Wayland clients can tell this stardust client to open a context menu for window management options.
	WindowMenu = 1,
	/// Maximize is supported, no shadows and the panel item UI controls the size of the window entirely.
	Maximize = 2,
	/// Fullscreen is supported, no shadows or title bar and the panel item UI controls the size of the window entirely.
	Fullscreen = 3,
	/// Minimize is supported, this just makes the button send the panel item handler an event when it's clicked.
	Minimize = 4,
}

/// The init data for the panel item.
#[derive(Debug, Clone, Deserialize)]
pub struct PanelItemInitData {
	/// The cursor, if applicable.
	pub cursor: Option<CursorInfo>,
	/// Size of the toplevel surface in pixels.
	pub toplevel: Option<ToplevelInfo>,
	/// Vector of popups that already exist
	pub popups: Vec<PopupInfo>,
	/// The surface, if any, that has exclusive input to the pointer.
	pub pointer_grab: Option<SurfaceID>,
	/// The surface, if any, that has exclusive input to the keyboard.
	pub keyboard_grab: Option<SurfaceID>,
}

/// An ID for a surface inside this panel item
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SurfaceID {
	Cursor,
	Toplevel,
	Popup(String),
}

impl<'de> serde::Deserialize<'de> for SurfaceID {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_seq(SurfaceIDVisitor)
	}
}

struct SurfaceIDVisitor;

impl<'de> Visitor<'de> for SurfaceIDVisitor {
	type Value = SurfaceID;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str("idk")
	}

	fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
		let Some(discrim) = seq.next_element()? else {
            return Err(A::Error::missing_field("discrim"));
        };

		// idk if you wanna check for extraneous elements
		// I didn't bother

		match discrim {
			"Cursor" => Ok(SurfaceID::Cursor),
			"Toplevel" => Ok(SurfaceID::Toplevel),
			"Popup" => {
				let Some(text) = seq.next_element()? else {
                    return Err(A::Error::missing_field("popup_text"));
                };
				Ok(SurfaceID::Popup(text))
			}
			_ => Err(A::Error::unknown_variant(discrim, &["Toplevel", "Popup"])),
		}
	}
}

impl serde::Serialize for SurfaceID {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self {
			Self::Cursor => ["Cursor"].serialize(serializer),
			Self::Toplevel => ["Toplevel"].serialize(serializer),
			Self::Popup(text) => ["Popup", text].serialize(serializer),
		}
	}
}

/// An item that represents a toplevel wayland surface (base window) and all its popups (context menus, modals, etc.).
#[derive(Debug)]
pub struct PanelItem {
	spatial: Spatial,
}
impl PanelItem {
	/// Apply a surface's visuals as a material to a model.
	///
	/// This material is unlit with the [Simula text shader](https://github.com/SimulaVR/Simula/blob/master/addons/godot-haskell-plugin/TextShader.tres) ported on the server.
	/// The material index is global across the whole model for now, just play around with it a bit.
	pub fn apply_surface_material(
		&self,
		surface: &SurfaceID,
		model_part: &ModelPart,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"apply_surface_material",
			&(surface, model_part.node().get_path()?),
		)
	}

	/// Request a resize of the surface (in pixels).
	///
	/// The surface's actual size after being resized will be given if the panel item is wrapped as `PanelItemHandler::resize`.
	pub fn set_toplevel_capabilities(&self, capabilities: &[Capability]) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_toplevel_capabilities", &capabilities)
	}

	/// Request a resize of the surface (in pixels).
	///
	/// The surface's actual size after being resized will be given if the panel item is wrapped as `PanelItemHandler::resize`.
	pub fn configure_toplevel(
		&self,
		size: Option<Vector2<u32>>,
		states: &[State],
		bounds: Option<Vector2<u32>>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("configure_toplevel", &(size, states, bounds))
	}

	/// Send an event to set the pointer's position (in pixels, relative to top-left of surface). This will activate the pointer.
	pub fn pointer_motion(
		&self,
		surface: &SurfaceID,
		position: impl Into<Vector2<f32>>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_motion", &(surface, position.into()))
	}
	/// Send an event to set a pointer button's state if the pointer's active.
	///
	/// The `button` is from the `input_event_codes` crate (e.g. BTN_LEFT for left click).
	pub fn pointer_button(
		&self,
		surface: &SurfaceID,
		button: u32,
		pressed: bool,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_button", &(surface, button, pressed as u32))
	}
	/// Send an event to scroll the pointer if it's active.
	///
	/// Scroll distance is a value in pixels corresponding to the "distance" the surface should be scrolled.
	/// Scroll steps is a value in columns/rows corresponding to the wheel clicks of a mouse or such. This also supports fractions of a wheel click.
	///
	/// If both the distance and steps are `None` then the scroll will be considered stopped. Either one being `Some` just scrolls.
	pub fn pointer_scroll(
		&self,
		surface: &SurfaceID,
		scroll_distance: Option<Vector2<f32>>,
		scroll_steps: Option<Vector2<f32>>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_scroll", &(surface, scroll_distance, scroll_steps))
	}

	/// Set the keyboard's keymap with a given `xkb` keymap.
	pub fn keyboard_set_keymap(&self, keymap: &str) -> Result<(), NodeError> {
		#[cfg(feature = "verify-keymap")]
		xkbcommon::xkb::Keymap::new_from_string(
			&xkbcommon::xkb::Context::new(0),
			keymap.to_string(),
			xkbcommon::xkb::KEYMAP_FORMAT_TEXT_V1,
			0,
		)
		.ok_or(NodeError::InvalidPath)?;
		self.node
			.send_remote_signal("keyboard_set_keymap_string", &keymap)
	}

	/// Set a key's state if the keyboard is active.
	///
	/// `key` is a raw keycode that corresponds to the given keymap.
	pub fn keyboard_key(
		&self,
		surface: &SurfaceID,
		key: u32,
		state: bool,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("keyboard_key", &(surface, key, state as u32))
	}

	fn handle_commit_toplevel<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		handler.lock().commit_toplevel(deserialize(data)?);
		Ok(())
	}
	fn handle_recommend_toplevel_state<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		handler.lock().recommend_toplevel_state(deserialize(data)?);
		Ok(())
	}
	fn handle_set_cursor<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		handler.lock().set_cursor(deserialize(data)?);
		Ok(())
	}

	fn handle_grab_keyboard<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		handler.lock().grab_keyboard_focus(deserialize(data)?);
		Ok(())
	}
	fn handle_grab_pointer<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		handler.lock().grab_pointer_focus(deserialize(data)?);
		Ok(())
	}

	fn handle_new_popup<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		let (uid, data): (&str, Option<PopupInfo>) = deserialize(data)?;
		handler.lock().new_popup(uid, data);
		Ok(())
	}
	fn handle_reposition_popup<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		let (uid, data): (&str, Option<PositionerData>) = deserialize(data)?;
		handler.lock().reposition_popup(uid, data);
		Ok(())
	}
	fn handle_drop_popup<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		handler.lock().drop_popup(deserialize(data)?);
		Ok(())
	}

	/// Wrap the panel item and `PanelItemHandler` in a `HandlerWrapper` to receive resize and cursor events.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: PanelItemHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		self.wrap_raw(Arc::new(Mutex::new(handler)))
	}
	/// Wrap the panel item and `PanelItemHandler` in a `HandlerWrapper` to receive resize and cursor events.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap_raw<H: PanelItemHandler>(
		self,
		handler: Arc<Mutex<H>>,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new_raw(self, handler);
		handler_wrapper
			.add_handled_signal("commit_toplevel", Self::handle_commit_toplevel)
			.unwrap();
		handler_wrapper
			.add_handled_signal(
				"recommend_toplevel_state",
				Self::handle_recommend_toplevel_state,
			)
			.unwrap();
		handler_wrapper
			.add_handled_signal("set_cursor", Self::handle_set_cursor)
			.unwrap();
		handler_wrapper
			.add_handled_signal("grab_keyboard", Self::handle_grab_keyboard)
			.unwrap();
		handler_wrapper
			.add_handled_signal("grab_pointer", Self::handle_grab_pointer)
			.unwrap();

		handler_wrapper
			.add_handled_signal("new_popup", Self::handle_new_popup)
			.unwrap();
		handler_wrapper
			.add_handled_signal("reposition_popup", Self::handle_reposition_popup)
			.unwrap();
		handler_wrapper
			.add_handled_signal("drop_popup", Self::handle_drop_popup)
			.unwrap();

		Ok(handler_wrapper)
	}
}
impl NodeType for PanelItem {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		PanelItem {
			spatial: self.spatial.alias(),
		}
	}
}
impl HandledNodeType for PanelItem {}
impl Item for PanelItem {
	type ItemType = PanelItem;
	type InitData = PanelItemInitData;
	const TYPE_NAME: &'static str = "panel";

	fn from_path(
		client: &Arc<Client>,
		parent_path: impl ToString,
		name: impl ToString,
		_init_data: &PanelItemInitData,
	) -> Self {
		// let handler_wrapper = HandlerWrapper::new(item, |handler: Weak<Mutex<T>>, item| {
		// 	ui_init_fn(init_data, handler, item)
		// });
		// handler_wrapper
		// 	.add_handled_signal("resize", Self::handle_resize)
		// 	.unwrap();
		// handler_wrapper
		// 	.add_handled_signal("set_cursor", Self::handle_set_cursor)
		// 	.unwrap();
		PanelItem {
			spatial: Spatial {
				node: Node::from_path(client, parent_path, name, false),
			},
		}
	}
}
impl Deref for PanelItem {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_panel_ui() {
	color_eyre::install().unwrap();
	use manifest_dir_macros::directory_relative_path;
	use rustc_hash::FxHashMap;
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	struct PanelItemManager(FxHashMap<String, HandlerWrapper<PanelItem, PanelItemUI>>);
	impl crate::items::ItemUIHandler<PanelItem> for PanelItemManager {
		fn item_created(&mut self, uid: &str, item: PanelItem, init_data: PanelItemInitData) {
			item.configure_toplevel(
				Some(Vector2::from([1000; 2])),
				&[State::Activated],
				Some(Vector2::from([500; 2])),
			)
			.unwrap();
			self.0.insert(
				uid.to_string(),
				item.wrap(PanelItemUI::new(init_data)).unwrap(),
			);
		}
		fn item_captured(&mut self, _uid: &str, acceptor_uid: &str, item: PanelItem) {
			println!(
				"Acceptor {} captured panel item {}",
				acceptor_uid,
				item.node().get_name().unwrap()
			);
		}
		fn item_released(&mut self, _uid: &str, acceptor_uid: &str, item: PanelItem) {
			println!(
				"Acceptor {} released panel item {}",
				acceptor_uid,
				item.node().get_name().unwrap()
			);
		}
	}
	struct PanelItemUI;
	impl PanelItemUI {
		fn new(init_data: PanelItemInitData) -> Self {
			println!("Panel item created with {:?}", init_data);
			PanelItemUI
		}
	}
	impl PanelItemHandler for PanelItemUI {
		fn commit_toplevel(&mut self, toplevel_state: Option<ToplevelInfo>) {
			dbg!(toplevel_state);
		}
		fn recommend_toplevel_state(&mut self, state: RequestedState) {
			dbg!(state);
		}
		fn show_window_menu(&mut self) {
			println!("Show window menu");
		}
		fn set_cursor(&mut self, cursor_info: Option<CursorInfo>) {
			dbg!(cursor_info);
		}
	}
	impl Drop for PanelItemUI {
		fn drop(&mut self) {
			println!("Panel item destroyed");
		}
	}

	let _item_ui = crate::items::ItemUI::<PanelItem>::register(&client)
		.unwrap()
		.wrap(PanelItemManager(FxHashMap::default()))
		.unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
