//! An example preprocessor for removing all forms of emphasis from a markdown
//! book.

extern crate comrak;
extern crate mdbook;

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

const NAME: &str = "md-links-to-html-links";

fn main() {
    panic!("This example is intended to be part of a library");
}

#[allow(dead_code)]
struct Deemphasize;

impl Preprocessor for Deemphasize {
    fn name(&self) -> &str {
        NAME
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        eprintln!("Running '{}' preprocessor", self.name());
        let mut num_removed_items = 0;

        process(&mut book.sections, &mut num_removed_items)?;

        eprintln!(
            "{}: removed {} events from markdown stream.",
            self.name(),
            num_removed_items
        );

        Ok(book)
    }
}

fn process<'a, I>(items: I, num_removed_items: &mut usize) -> Result<()>
where
    I: IntoIterator<Item = &'a mut BookItem> + 'a,
{
    for item in items {
        if let BookItem::Chapter(ref mut chapter) = *item {
            eprintln!("{}: processing chapter '{}'", NAME, chapter.name);

            let md = remove_emphasis(num_removed_items, chapter)?;
            chapter.content = md;
        }
    }

    Ok(())
}

fn remove_emphasis(num_removed_items: &mut usize, chapter: &mut Chapter) -> Result<String> {
    // Parse the markdown to an AST, modify it, and then re-render it as markdown.
    use comrak::nodes::{AstNode, NodeValue};

    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, &chapter.content, &comrak::ComrakOptions::default());

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F)
    where
        F: FnMut(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &mut |node| {
        if let NodeValue::Emph = node.data.borrow().value {
            *num_removed_items += 1;
            // Replace this node with its children, which is a Text node.
            for child in node.children() {
                node.insert_before(child);
            }
            node.detach();
        }
    });

    let mut new_markdown = vec![];
    comrak::format_commonmark(root, &comrak::ComrakOptions::default(), &mut new_markdown)?;
    let new_markdown = String::from_utf8(new_markdown)?;
    Ok(new_markdown)
}
