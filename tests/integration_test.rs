use skyscraper::html::{DocumentNode, HtmlNode};
use skyscraper::{html, xpath};

static HTML: &'static str = include_str!("samples/James-LG_Skyscraper.html");

#[test]
fn xpath_github_sample1() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = skyscraper::xpath::parse::parse("//main").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(1, nodes.len());

    let doc_node = nodes[0];
    match document.get_html_node(&doc_node).unwrap() {
        HtmlNode::Tag(t) => assert_eq!("main", t.name),
        HtmlNode::Text(_) => panic!("expected tag, got text instead"),
    }
}

#[test]
fn xpath_github_sample2() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse::parse("//a[@class='Link--secondary']").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(5, nodes.len());

    let doc_node = nodes[0];
    match document.get_html_node(&doc_node).unwrap() {
        HtmlNode::Tag(t) => {
            assert_eq!("a", t.name);

            let children: Vec<DocumentNode> = doc_node.children(&document).collect();
            assert_eq!(1, children.len());

            match document.get_html_node(&children[0]).unwrap() {
                HtmlNode::Tag(_) => panic!("expected text, got tag instead"),
                HtmlNode::Text(text) => assert_eq!("refactor: Reorganize into workspace", text),
            }
        }
        HtmlNode::Text(_) => panic!("expected tag, got text instead"),
    }
}

#[test]
fn xpath_github_sample3() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath =
        xpath::parse("//div[@class='BorderGrid-cell']/div[@class=' text-small']/a").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(1, nodes.len());

    let doc_node = nodes[0];
    match document.get_html_node(&doc_node).unwrap() {
        HtmlNode::Tag(t) => {
            assert_eq!("a", t.name);

            let children: Vec<DocumentNode> = doc_node.children(&document).collect();
            assert_eq!(1, children.len());

            match document.get_html_node(&children[0]).unwrap() {
                HtmlNode::Tag(_) => panic!("expected text, got tag instead"),
                HtmlNode::Text(text) => assert_eq!("Create a new release", text),
            }
        }
        HtmlNode::Text(_) => panic!("expected tag, got text instead"),
    }
}

#[test]
fn xpath_github_get_text_sample() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("//div[@class='flex-auto min-width-0 width-fit mr-3']").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(1, nodes.len());

    let doc_node = nodes[0];
    let html_node = document.get_html_node(&doc_node).unwrap();
    let text = html_node.get_all_text(&doc_node, &document).unwrap();

    assert_eq!("James-LG / Skyscraper Public", text);
}

#[test]
fn xpath_github_parent_axis() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("//div[@role='gridcell']/parent::div").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(5, nodes.len());
}

#[test]
fn xpath_github_parent_axis_recursive() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("//div[@role='gridcell']//parent::div").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(20, nodes.len());
}

#[test]
fn xpath_github_dashed_attribute() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("//span[@data-view-component='true']").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(19, nodes.len());
}

#[test]
fn xpath_github_get_attributes_sample() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("//div[@class='flex-auto min-width-0 width-fit mr-3']").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(1, nodes.len());

    let doc_node = nodes[0];
    let html_node = document.get_html_node(&doc_node).unwrap();
    let attributes = html_node.get_attributes().unwrap();
    assert_eq!("flex-auto min-width-0 width-fit mr-3", attributes["class"]);
}

#[test]
fn xpath_github_root_search() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("/html").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(1, nodes.len());

    let doc_node = nodes[0];
    let html_node = document.get_html_node(&doc_node).unwrap();
    let tag = html_node.unwrap_tag();
    assert_eq!("html", tag.name);
}

#[test]
fn xpath_github_root_search_all() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("//html").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(1, nodes.len());

    let doc_node = nodes[0];
    let html_node = document.get_html_node(&doc_node).unwrap();
    let tag = html_node.unwrap_tag();
    assert_eq!("html", tag.name);
}

#[test]
fn xpath_github_root_wildcard() {
    // arrange
    let text: String = HTML.parse().unwrap();

    let document = html::parse(&text).unwrap();
    let xpath = xpath::parse("//body/*").unwrap();

    // act
    let nodes = xpath.apply(&document).unwrap();

    // assert
    assert_eq!(16, nodes.len());

    // assert first node
    let doc_node = nodes[0];
    let html_node = document.get_html_node(&doc_node).unwrap();
    let tag = html_node.unwrap_tag();

    assert_eq!("div", tag.name);

    let attributes = html_node.get_attributes().unwrap();
    assert_eq!("position-relative js-header-wrapper ", attributes["class"]);

    // assert random node 4
    let doc_node = nodes[3];
    let html_node = document.get_html_node(&doc_node).unwrap();
    let tag = html_node.unwrap_tag();

    assert_eq!("include-fragment", tag.name);

    // assert last node
    let doc_node = nodes[15];
    let html_node = document.get_html_node(&doc_node).unwrap();
    let tag = html_node.unwrap_tag();

    assert_eq!("div", tag.name);

    let attributes = html_node.get_attributes().unwrap();
    assert_eq!("sr-only", attributes["class"]);
    assert_eq!("polite", attributes["aria-live"])
}
