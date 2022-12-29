#![allow(incomplete_features)]

use cursive::event::{Event, EventResult, Key};
use cursive::view::ViewWrapper;

use cursive::wrap_impl;
use cursive::{
    traits::*,
    views::{Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView, TextView},
};

fn main() {
    let mut siv = cursive::default();

    let mut month_select = SelectView::new().autojump().popup();

    for i in 1..13 {
        month_select.add_item(format!("{:02}", i), i);
    }

    let mut year_select = SelectView::new().autojump().popup();

    for i in 2022..2031 {
        year_select.add_item(format!("{:02}", i), i);
    }

    let mut cc_number = EditView::new()
        .with(move | v| v.set_on_edit(| _s, _, _|{
            // v.set_content(format!("Test"));
        }))
        .wrap_with(InputMaskView::new);
    //cc_number.set_on_edit_mut(cc_number_on_edit);

    let cc_number = cc_number
        //.on_edit(cc_number_on_edit)
        .fixed_width(18)
        .wrap_with(cursive::views::FocusTracker::new)
        .on_focus(|_| {
            cursive::event::EventResult::with_cb(|s| {
                s.call_on_name("card_type", |v: &mut cursive::views::TextView| {
                    v.set_content("?");
                });
            })
        })
        .on_focus_lost(|_| {
            cursive::event::EventResult::with_cb(|s| {
                s.call_on_name("card_type", |v: &mut cursive::views::TextView| {
                    v.set_content("V");
                });
            })
        });

    //.wrap_with(cursive::views::OnEventView::new)
    // .wrap_with(cursive::views::OnEventView::new)

    // let cc = OnEventView::new(cc)
    //     .on_event(event::Key::Esc, |s| s.quit())
    //     .on_event('q', |s| s.quit());

    let cc_view = LinearLayout::horizontal()
        .child(cc_number.with_name("cc_number"))
        .child(TextView::new(" "))
        .child(TextView::new(" ").with_name("card_type"))
        // apparently #on_focus_lost requires a View that implements #tale_focus, apparently.
        // temp add a View (Button) that can take focus so that the event can trigger.
        .child(cursive::views::Button::new(" ", |s| s.quit()).with_name("xyz"));

    siv.add_layer(
        Dialog::new()
            .title("Provide payment information")
            .button("Ok", |s| s.quit())
            .content(
                ListView::new()
                    // Each child is a single-line view with a label
                    .child(
                        "Name",
                        EditView::new()
                            .content("Some Name")
                            .fixed_width(30)
                            .with_name("cardholder_name"),
                    )
                    .delimiter()
                    .child("Card Number", cc_view)
                    .delimiter()
                    .child("Parsed CC", EditView::new().with_name("parsed_card"))
                    .delimiter()
                    .child(
                        "Receive spam?",
                        Checkbox::new().on_change(|s, checked| {
                            // Enable/Disable the next field depending on this checkbox
                            for name in &["email1", "email2"] {
                                s.call_on_name(name, |view: &mut EditView| {
                                    view.set_enabled(checked)
                                });
                                if checked {
                                    s.focus_name("email1").unwrap();
                                }
                            }
                        }),
                    )
                    // Delimiter currently are just a blank line
                    .delimiter()
                    .child(
                        "Expiration",
                        // Popup-mode SelectView are small enough to fit here
                        LinearLayout::horizontal()
                            .child(month_select.with_name("mm"))
                            .child(TextView::new("/"))
                            .child(year_select.with_name("yyyy")),
                    )
                    .scrollable(),
            ),
    );

    siv.run();
}

struct InputMaskView<T> {
    view: T,
}

impl<T> InputMaskView<T>
where
    T: View,
{
    fn new(view: T) -> Self {
        InputMaskView { view }
    }
}

impl<T: View> ViewWrapper for InputMaskView<T> {
    wrap_impl!(self.view: T);

    // fn wrap_on_event(&mut self, event: Event) -> EventResult {
    //     match event {
    //         Event::Char('1')
    //         | Event::Char('2')
    //         | Event::Char('3')
    //         | Event::Char('4')
    //         | Event::Char('5')
    //         | Event::Char('6')
    //         | Event::Char('7')
    //         | Event::Char('8')
    //         | Event::Char('9')
    //         | Event::Char('0')
    //         | Event::Char(' ')
    //         | Event::Key(Key::Del)
    //         | Event::Key(Key::Backspace) => {
    //             // from EditView
    //             self.view.on_event(event);
    //             // return EventResult::Consumed(Some());
    //         },
    //         Event::Char('a') => {
    //             self.view.on_event(Event::Char('A'));
    //         }
    //         _ => {},
    //     }
    //     EventResult::Ignored
    // }

    // Destructuring ! https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-nested-structs-and-enums
    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        return match event {
            Event::Char(char) => match char {
                'a'..='c' => self.view.on_event(Event::Char('A')), // easter egg
                '0'..='9' | ' ' => self.view.on_event(event),
                _ => EventResult::Ignored,
            },
            Event::Key(Key::Del) | Event::Key(Key::Backspace) => self.view.on_event(event),
            _ => EventResult::Ignored,
        };
    }
}
