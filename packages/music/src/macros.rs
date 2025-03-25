macro_rules! chord {
   ($first:ident, $second:ident $(, $rest:ident)*)  => {{
       crate::Chord::try_from(notes!($first, $second $(, $rest)*)).unwrap()
   }};

   ($first:ident, $second:ident $(, $rest:ident)*; $extension:ident)  => {{
       let mut chord = chord!($first, $second $(, $rest)*);
       chord.insert($extension);
       chord
   }};
}

macro_rules! notes {
    ($($note:ident),*) => {{
        let mut notes = crate::Notes::empty();
        $(notes.insert(crate::Note::$note);)*
        notes
    }};

}
