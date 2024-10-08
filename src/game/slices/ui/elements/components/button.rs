use crate::game::slices::ui::Element;

pub fn create_base_element() -> Element {
  Element {
    ..Default::default()
  }
}
