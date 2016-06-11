macro_rules! struct_events {
    (
        keyboard: { $($k_alias:ident : $k_sdl:ident ),*, },

        else: { $( $e_alias:ident : $e_sdl:pat ),*, }
    ) => {

        #[derive(Debug)]
        pub struct ImmediateEvents {
            $( pub $k_alias: Option<Key> ),*,
            $( pub $e_alias: bool),*,
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    $( $k_alias: None ),*,
                    $( $e_alias: false),*,
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            $( pub $k_alias: Key ),*,
            $( pub $e_alias: bool ),*,
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    $( $k_alias: Key::Up ),*,
                    $( $e_alias: false ),*,
                }
            }

            pub fn pump(&mut self) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        KeyDown { keycode, .. } => match keycode {
                            // $( ... ),* containing $k_sdl and $k_alias means:
                            //   "for every element ($k_alias : $k_sdl) pair,
                            //    check whether the keycode is Some($k_sdl). If
                            //    it is, then set the $k_alias fields to true."
                            $(
                                Some($k_sdl) => {
                                    // Prevent multiple presses when keeping a key down
                                    if let Key::Up = self.$k_alias {
                                        self.now.$k_alias = Some(Key::Down);
                                    }

                                    self.$k_alias = Key::Down;
                                }
                                ),*
                                _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Key released
                                    self.now.$k_alias = Some(Key::Up);
                                    self.$k_alias = Key::Up;
                                }
                                ),*
                                _ => {}
                        },
                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                            )*,

                        _ => {}
                    }
                }
            }
        }
    }
}
