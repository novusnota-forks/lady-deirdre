use lady_deirdre::{
    arena::Ref,
    lexis::{CodeContent, ToSpan},
    syntax::{SyntaxTree, TreeContent},
    Document,
};
use lady_deirdre_examples::json::syntax::JsonNode;

#[test]
fn test_clusters_traverse() {
    let mut doc = Document::<JsonNode>::default();

    assert_eq!(
        0..0,
        doc.root_node_ref()
            .cluster()
            .span(&doc)
            .to_span(&doc)
            .unwrap()
    );

    doc.write(
        ..,
        r#"{"foo": [1, 3, true, false, null, {"a": "xyz", "b": null}], "baz": {}}"#,
    );

    assert_eq!(
        0..70,
        doc.root_node_ref()
            .cluster()
            .span(&doc)
            .to_span(&doc)
            .unwrap()
    );

    assert_eq!(
        0..70,
        doc.get_cluster_span(&Ref::Primary).to_span(&doc).unwrap()
    );

    let mut cluster = doc.root_node_ref().cluster();

    cluster = cluster.next(&doc);
    assert_eq!(1..58, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(8..58, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(9..10, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(12..13, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(15..19, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(21..26, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(28..32, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(34..57, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(35..45, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(40..45, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(47..56, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(52..56, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(60..69, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.next(&doc);
    assert_eq!(67..69, cluster.span(&doc).to_span(&doc).unwrap());

    assert!(!cluster.next(&doc).is_valid_ref(&doc));

    cluster = cluster.previous(&doc);
    assert_eq!(60..69, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(52..56, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(47..56, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(40..45, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(35..45, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(34..57, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(28..32, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(21..26, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(15..19, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(12..13, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(9..10, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(8..58, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(1..58, cluster.span(&doc).to_span(&doc).unwrap());

    cluster = cluster.previous(&doc);
    assert_eq!(0..70, cluster.span(&doc).to_span(&doc).unwrap());

    assert!(!cluster.previous(&doc).is_valid_ref(&doc));
}

#[test]
fn test_clusters_cover() {
    let doc = Document::<JsonNode>::from(
        r#"{"foo": [1, 3, true, false, null, {"a": "xyz", "b": null}], "baz": {}}"#,
    );

    assert_eq!(
        r#"{"foo": [1, 3, true, false, null, {"a": "xyz", "b": null}], "baz": {}}"#,
        doc.substring(doc.cover(..).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#"{"foo": [1, 3, true, false, null, {"a": "xyz", "b": null}], "baz": {}}"#,
        doc.substring(doc.cover(0..0).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#"{"foo": [1, 3, true, false, null, {"a": "xyz", "b": null}], "baz": {}}"#,
        doc.substring(doc.cover(1..1).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#""foo": [1, 3, true, false, null, {"a": "xyz", "b": null}]"#,
        doc.substring(doc.cover(2..2).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#""foo": [1, 3, true, false, null, {"a": "xyz", "b": null}]"#,
        doc.substring(doc.cover(3..3).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#""foo": [1, 3, true, false, null, {"a": "xyz", "b": null}]"#,
        doc.substring(doc.cover(8..8).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#"[1, 3, true, false, null, {"a": "xyz", "b": null}]"#,
        doc.substring(doc.cover(9..9).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#"[1, 3, true, false, null, {"a": "xyz", "b": null}]"#,
        doc.substring(doc.cover(15..15).span(&doc).to_span(&doc).unwrap())
    );

    assert_eq!(
        r#"true"#,
        doc.substring(doc.cover(16..16).span(&doc).to_span(&doc).unwrap())
    );
}
