fn format_string(padding: usize, s: &str) -> String {
    let s_len = s.len();

    if padding == s_len {
        format!(" {} ", s)
    } else if padding < s_len {
        format!(
            " {}... ",
            &s[..(if s_len < padding - 3 {
                s_len
            } else {
                padding - 3
            })]
        )
    } else {
        format!(
            "{}{}{}",
            ".".repeat((padding as f32 - s_len as f32).floor() as usize / 2 + 1),
            s,
            ".".repeat((padding as f32 - s_len as f32).ceil() as usize / 2 + 1)
        )
    }
}

fn calculate_table_width(items: &[&str], p: usize) -> usize {
    let mut table_width = items.len() + 1;

    let element_width = |s_len| if s_len <= p - 3 { s_len + 3 + 2 } else { p + 2 };

    items
        .iter()
        .for_each(|i| table_width += element_width(i.len()));

    table_width
}

fn print_horizontal_border(table_w: usize, sep: char) {
    for _ in 0..table_w {
        print!("{}", sep);
    }
}

fn print_items(items: Vec<String>, sep: char, outer: bool) {
    if outer {
        print!("|")
    }
    for (i, item) in items.iter().enumerate() {
        print!(
            "{}{}",
            item,
            (if i != items.len() - 1 || (items.len() - 1 == i && outer) {
                "|"
            } else {
                ""
            })
        );
    }
}

fn print_table(head: &[&str], padding: usize, print_outer_border: bool, sep: char) {
    let tabel_width = calculate_table_width(head, padding);
    print_horizontal_border(tabel_width, '+');
    println!("");
    print_items(
        head.iter().map(|e| format_string(padding, e)).collect(),
        sep,
        print_outer_border,
    );
    println!("");
    print_horizontal_border(tabel_width, '+');
}

fn main() {
    //    print_table(&["id", "printibale", "good", "clr", "x"], 4, true, '|')
    let slice: &str = "anas jaidi";

    let os = Some(slice);

    let a = &os;

    if let Some(s) = &os {
        let r = &(*s)[0..1];
    }
}
