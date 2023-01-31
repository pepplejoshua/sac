use super::parser::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    name: String,
    attrs: Vec<(String, String)>,
    children: Vec<Element>,
}

#[allow(dead_code)]
fn quoted_string<'a>() -> impl Parser<'a, String> {
    right(
        literal("\""),
        left(zero_or_more(any_char.pred(|c| *c != '"')), literal("\"")),
    )
    .map(|chars| chars.into_iter().collect())
}

#[test]
fn test_quoted_string() {
    assert_eq!(
        Ok(("", "Hello, World!".into())),
        quoted_string().parse("\"Hello, World!\"")
    )
}

#[allow(dead_code)]
fn attribute_pair<'a>() -> impl Parser<'a, (String, String)> {
    pair(identifier, right(literal("="), quoted_string()))
}

#[allow(dead_code)]
fn attributes<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    zero_or_more(right(space1(), attribute_pair()))
}

#[test]
fn test_attributes() {
    let parser = attributes();
    assert_eq!(
        parser.parse(" name=\"Joshua\" age=\"23\""),
        Ok((
            "",
            vec![
                ("name".into(), "Joshua".into()),
                ("age".into(), "23".into())
            ]
        ))
    )
}

#[allow(dead_code)]
fn element_start<'a>() -> impl Parser<'a, (String, Vec<(String, String)>)> {
    right(literal("<"), pair(identifier, attributes()))
}

#[allow(dead_code)]
fn single_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), literal("/>")).map(|(name, attrs)| Element {
        name,
        attrs,
        children: vec![],
    })
}

#[test]
fn test_single_element() {
    let parser = single_element();
    let html = r#"<a id="go_to_google" href="https://www.google.com" onClick="doSomething()"/>"#;
    assert_eq!(
        parser.parse(html),
        Ok((
            "",
            Element {
                name: "a".into(),
                attrs: vec![
                    ("id".into(), "go_to_google".into()),
                    ("href".into(), "https://www.google.com".into()),
                    ("onClick".into(), "doSomething()".into()),
                ],
                children: vec![],
            }
        ))
    )
}

#[allow(dead_code)]
fn open_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), literal(">")).map(|(name, attrs)| Element {
        name,
        attrs,
        children: vec![],
    })
}

#[allow(dead_code)]
fn close_element<'a>(expected_name: String) -> impl Parser<'a, String> {
    right(literal("</"), left(identifier, literal(">")))
        .pred(move |parsed_name| parsed_name == &expected_name)
}

#[allow(dead_code)]
fn parent_element<'a>() -> impl Parser<'a, Element> {
    open_element().and_then(|el| {
        left(zero_or_more(element()), close_element(el.name.clone())).map(move |children| {
            let mut elem = el.clone();
            elem.children = children;
            elem
        })
    })
}

#[allow(dead_code)]
fn element<'a>() -> impl Parser<'a, Element> {
    whitespace_wrap(either(single_element(), parent_element()))
}

#[test]
fn test_xml_parser() {
    let doc = r#"
        <top label="Top">
            <semi-bottom label="Bottom"/>
            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"#;

    let parsed_doc = Element {
        name: "top".into(),
        attrs: vec![("label".into(), "Top".into())],
        children: vec![
            Element {
                name: "semi-bottom".into(),
                attrs: vec![("label".into(), "Bottom".into())],
                children: vec![],
            },
            Element {
                name: "middle".into(),
                attrs: vec![],
                children: vec![Element {
                    name: "bottom".into(),
                    attrs: vec![("label".into(), "Another bottom".into())],
                    children: vec![],
                }],
            },
        ],
    };

    assert_eq!(Ok(("", parsed_doc)), element().parse(doc));
}
