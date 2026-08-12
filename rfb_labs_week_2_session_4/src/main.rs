//! Small executable for Part 8 of the assignment.

use rfb_labs_week_2_session_4::LibraryError;
use rfb_labs_week_2_session_4::catalogue::{Item, MediaKind};
use rfb_labs_week_2_session_4::library::Library;
use rfb_labs_week_2_session_4::member::Member;

fn main() -> Result<(), LibraryError> {
    let mut library = Library::new();

    // Stock the library
    library.add_item(Item::new(
        1,
        "Dune".into(),
        "Frank Herbert".into(),
        MediaKind::Book { pages: 320 },
    ))?;
    library.add_item(Item::new(
        2,
        "Project Hail Mary".into(),
        "Andy Weir".into(),
        MediaKind::Audiobook { minutes: 540 },
    ))?;
    library.add_item(Item::new(
        3,
        "The Rust Programming Language".into(),
        "Steve Klabnik".into(),
        MediaKind::Ebook { size_kb: 1_200 },
    ))?;

    // Register a member
    library.register_member(Member::new(100, "Ada".into()))?;

    println!("=== Library stocked ===");
    for item in library.available_items() {
        println!("  {item}");
    }

    // Successful checkout — borrow Dune on day 0
    library.checkout(1, 100, 0)?;
    println!("\nAda checks out item 1 on day 0.");
    println!("  Item status: {}", library.find_item(1).unwrap().status);

    // Late return — Dune is due after 21 days; returned on day 30 (9 days late)
    let fee = library.return_item(1, 30)?;
    println!("\nAda returns item 1 on day 30.");
    println!("  Late fee owed: {fee} cents ({} days overdue)", fee / 25);
    println!("  Item status: {}", library.find_item(1).unwrap().status);

    // Demonstrate a handled error — try to check out an item that doesn't exist
    match library.checkout(99, 100, 30) {
        Err(e) => println!("\nHandled error: {e}"),
        Ok(_) => unreachable!(),
    }

    Ok(())
}
