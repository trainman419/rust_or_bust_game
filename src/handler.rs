extern crate piston_window;

use error;

/// An interface describing all the different input-events that can be handled.
pub trait InputHandler {
  fn on_button<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _button_args: &piston_window::ButtonArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_controller_axis<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _controller_axis_args: &piston_window::ControllerAxisArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_mouse_cursor<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _position: &[f64; 2],
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_mouse_relative<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _relative: &[f64; 2],
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_mouse_scroll<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _scroll: &[f64; 2],
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_press<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _button: &piston_window::Button,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_release<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _button: &piston_window::Button,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_text<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _text: &String,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_touch<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _touch_args: &piston_window::TouchArgs,
  ) -> error::Result<()> {
    Ok(())
  }
}

/// An interface describing all the different update-events that can be handled.
pub trait UpdateHandler {
  fn on_idle<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _idle_args: &piston_window::IdleArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_update<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _update_args: &piston_window::UpdateArgs,
  ) -> error::Result<()> {
    Ok(())
  }
}

/// An interface describing all the different window-events that can be handled.
pub trait WindowHandler {
  fn on_after_render<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _after_render_args: &piston_window::AfterRenderArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_close<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _close_args: &piston_window::CloseArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_cursor<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _cursor: bool,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_focus<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _focus: bool,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_render<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _render_args: &piston_window::RenderArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_resize<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _size: &[u32; 2],
  ) -> error::Result<()> {
    Ok(())
  }
}

/// An interface that dispatches events to more specific handlers.
pub trait EventHandler: InputHandler + UpdateHandler + WindowHandler {
  fn on_event<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
  ) -> error::Result<()> {
    // Dispatch input events to InputHandler functions.
    if let Some(button) = event.button_args() {
      self.on_button::<Event>(&event, &button)?;
    }
    if let Some(controller_axis) = event.controller_axis_args() {
      self.on_controller_axis::<Event>(&event, &controller_axis)?;
    }
    if let Some(mouse_cursor) = event.mouse_cursor_args() {
      self.on_mouse_cursor::<Event>(&event, &mouse_cursor)?;
    }
    if let Some(mouse_relative) = event.mouse_relative_args() {
      self.on_mouse_relative::<Event>(&event, &mouse_relative)?;
    }
    if let Some(mouse_scroll) = event.mouse_scroll_args() {
      self.on_mouse_scroll::<Event>(&event, &mouse_scroll)?;
    }
    if let Some(press) = event.press_args() {
      self.on_press::<Event>(&event, &press)?;
    }
    if let Some(release) = event.release_args() {
      self.on_release::<Event>(&event, &release)?;
    }
    if let Some(text) = event.text_args() {
      self.on_text::<Event>(&event, &text)?;
    }
    if let Some(touch) = event.touch_args() {
      self.on_touch::<Event>(&event, &touch)?;
    }

    // Dispatch update events to UpdateHandler functions.
    if let Some(idle) = event.idle_args() {
      self.on_idle::<Event>(&event, &idle)?;
    }
    if let Some(update) = event.update_args() {
      self.on_update::<Event>(&event, &update)?;
    }

    // Dispatch window events to WindowHandler functions.
    if let Some(after_render) = event.after_render_args() {
      self.on_after_render::<Event>(&event, &after_render)?;
    }
    if let Some(close) = event.close_args() {
      self.on_close::<Event>(&event, &close)?;
    }
    if let Some(cursor) = event.cursor_args() {
      self.on_cursor::<Event>(&event, cursor)?;
    }
    if let Some(focus) = event.focus_args() {
      self.on_focus::<Event>(&event, focus)?;
    }
    if let Some(render) = event.render_args() {
      self.on_render::<Event>(&event, &render)?;
    }
    if let Some(resize) = event.resize_args() {
      self.on_resize::<Event>(&event, &resize)?;
    }

    Ok(())
  }
}
