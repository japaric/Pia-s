use crate::back::chord_namer::NoteGrouper;
use crate::front::canvas::Canvas;
use crate::front::console::Console;
use crate::front::settings::midi_input::device::MidiInputDeviceSelect;

#[spur::subscriptions]
const _: Broker = {
    /* front */
    #[subscribed(to = crate::front::settings::midi_input::device::Initialize)]
    #[subscribed(to = crate::front::settings::midi_input::device::SelectChanged)]
    const _: MidiInputDeviceSelect = MidiInputDeviceSelect::new();

    #[subscribed(to = crate::front::canvas::Initialize)]
    #[subscribed(to = crate::messages::NewScaleTonicSelected)]
    #[subscribed(to = crate::messages::NewScaleTypeSelected)]
    #[subscribed(to = crate::messages::ActiveNotesChanged)]
    const _: Canvas = Canvas::new();

    #[subscribed(to = crate::front::console::Initialize)]
    #[subscribed(to = crate::messages::NewScaleTonicSelected)]
    #[subscribed(to = crate::messages::ActiveNotesChanged)]
    const _: Console = Console::new();

    /* back */
    #[subscribed(to = crate::messages::NoteOff)]
    #[subscribed(to = crate::messages::NoteOn)]
    #[subscribed(to = crate::messages::HoldPedalPressed)]
    #[subscribed(to = crate::messages::HoldPedalReleased)]
    const _: NoteGrouper = NoteGrouper::new();
};
