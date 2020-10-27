use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;
use std::cmp::Ordering;

use cursive_table_view::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
  Tidspunkt,
  Tittel,
}

#[derive(Clone, Debug)]
struct Programpost {
  tidspunkt: String,
  tittel: String,
  beskrivelse: String,
}

impl TableViewItem<BasicColumn> for Programpost {
  fn to_column(&self, column: BasicColumn) -> String {
    match column {
      BasicColumn::Tidspunkt => self.tidspunkt.to_string(),
      BasicColumn::Tittel => self.tittel.to_string(),
    }
  }

  fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
  where
    Self: Sized,
  {
    match column {
      BasicColumn::Tidspunkt => self.tidspunkt.cmp(&other.tidspunkt),
      BasicColumn::Tittel => self.tittel.cmp(&other.tittel),
    }
  }
}

fn main() {
  // Creates the cursive root - required for every application.
  let mut siv = cursive::default();

  siv.add_global_callback('q', |s| s.quit());

  siv.add_layer(
    Dialog::around(TextView::new(
      "Velkommen til faggruppefagdag med Utvikleropplevelse!",
    ))
    .title("Fagdag!")
    .button("Se program", |s| show_program(s)),
  );

  siv.run();
}

fn show_program(s: &mut Cursive) {
  let programposter = [
    Programpost {
      tidspunkt: String::from("09:00"),
      tittel: String::from("Start!"),
      beskrivelse: String::from("Oppvarmingsøvelse"),
    },
    Programpost {
      tidspunkt: String::from("09:30"),
      tittel: String::from("Config"),
      beskrivelse: String::from(
        "Vi ser gjennom config-filene våre og deler tips & tricks rundt terminal/shell og tooling",
      ),
    },
    Programpost {
      tidspunkt: String::from("10:00"),
      tittel: String::from("Gitttt"),
      beskrivelse: String::from(
        "Vi går sammen to og to og prøver oss på https://gitexercises.fracz.com/",
      ),
    },
    Programpost {
      tidspunkt: String::from("12:00"),
      tittel: String::from("Lønsj!"),
      beskrivelse: String::from("Digge smørbrød fra WB Samson"),
    },
    Programpost {
      tidspunkt: String::from("12:30"),
      tittel: String::from("Filmformiddag"),
      beskrivelse: String::from("Vi ser på et par-tre foredrag sammen"),
    },
    Programpost {
      tidspunkt: String::from("13:30"),
      tittel: String::from("Distribuert jobbing"),
      beskrivelse: String::from(
        "Øyvind presenterer resultat fra spørreundersøkelsen, og så diskuterer vi rundt det",
      ),
    },
  ];

  let mut table = TableView::<Programpost, BasicColumn>::new()
    .column(BasicColumn::Tidspunkt, "Tidspunkt", |c| {
      c.align(HAlign::Center)
    })
    .column(BasicColumn::Tittel, "Tittel", |c| c);

  table.set_items(programposter.to_vec());
  table.set_on_submit(move |s_, _row, index| show_programpost_details(s_, &programposter[index]));

  s.pop_layer();
  s.add_layer(
    LinearLayout::vertical()
      .child(Dialog::around(table.with_name("table").min_size((200, 20))).title("Program"))
      .child(TextView::new("Trykk <q> for å avslutte")),
  );
}

fn show_programpost_details(s: &mut Cursive, post: &Programpost) {
  s.add_layer(
    Dialog::text(format!("{}", &post.beskrivelse))
      .title(&post.tittel)
      .button("Tilbake til programmet", |s| {
        s.pop_layer();
      }),
  );
}
