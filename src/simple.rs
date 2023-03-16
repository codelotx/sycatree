use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub children: Vec<Node>,
}

impl Node {
    fn new(id: i32, parent_id: Option<i32>, name: &str) -> Node {
        Node {
            id,
            parent_id,
            name: name.to_owned(),
            children: vec![],
        }
    }

    fn has_child(&self, nodes: &[Node]) -> bool {
        nodes.iter().any(|n| n.parent_id == Some(self.id))
    }

    pub fn get_child_node(&self, id: i32) -> Option<Node> {
        for child in self.children.iter() {
            if child.id == id {
                return Some(child.clone());
            }
            if let Some(node) = child.get_child_node(id) {
                return Some(node);
            }
        }
        None
    }

    pub fn get_immediate_children<'a>(&'a self, nodes: &'a [Node]) -> Vec<&'a Node> {
        nodes
            .iter()
            .filter(|n| n.parent_id == Some(self.id))
            .collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct NodeList {
    pub list: Vec<Node>,
}

#[derive(Debug, Default, Clone)]
pub struct NodeState {
    pub nodes: RcSignal<NodeList>,
}

impl NodeList {
    pub fn get_root_nodes(&self) -> Vec<Node> {
        let mut root_nodes = Vec::new();
        for node in self.list.iter() {
            if node.parent_id.is_none() {
                root_nodes.push(node.clone());
            }
        }
        root_nodes
    }
}

#[component]
fn NestedNode<G: Html>(cx: Scope, n: Node) -> View<G> {
    let nm = n.clone();
    let all_nodes = use_context::<NodeState>(cx);
    let nodes = all_nodes.nodes.get().as_ref().clone().list;
    let ch = n
        .get_immediate_children(&nodes)
        .into_iter()
        .cloned()
        .collect();
    let chd = create_signal(cx, ch);
    let toggle_state = create_signal(cx, false);
    let toggle = |_| {
        if *toggle_state.get() {
            toggle_state.set(false)
        } else {
            toggle_state.set(true)
        }
    };
    let class = move || {
        format!(
            "fa-regular {}",
            if *toggle_state.get() {
                "fa-square-minus"
            } else if n.has_child(&nodes) {
                "fa-square-plus"
            } else {
                ""
            }
        )
    };

    view! {cx,  i(on:click=toggle, class=class()) (nm.name)
     (if *toggle_state.get() {

         view! { cx,
            ul(class="list-group") {
                Keyed(
                    iterable=chd,
                    view=|cx, x| view! { cx,
                       li(class="list-group-item") {NestedNode(x)}
                    },
                    key=|x| x.id,
                )
            }
        }

    } else {
        view! { cx, } // Now you don't
    }
     )
        }
}

#[component]
fn TreeNode<G: Html>(cx: Scope, n: Node) -> View<G> {
    let all_nodes = use_context::<NodeState>(cx);
    let nodes = all_nodes.nodes.get().as_ref().clone().list;
    let x_child = n.clone();

    let n_haschild = x_child.has_child(&nodes);
    view! { cx,
            li(class="list-group-item") {
                    (if n_haschild {
                        let nx = n.clone();


                   view!(cx, NestedNode(nx))
                    } else {
                        let nx = n.clone();
                        view! { cx,  (nx.name)}
                    })


            }

    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let vec_nodes = vec![
        Node::new(1, None, "Node 1"),
        Node::new(2, Some(1), "Node 2"),
        Node::new(3, Some(2), "Node 3"),
        Node::new(4, Some(3), "Node 4"),
        Node::new(5, Some(1), "Node 5"),
        Node::new(6, None, "Node 6"),
        Node::new(7, None, "Node 7"),
        Node::new(8, None, "Node 8"),
        Node::new(9, Some(7), "Node 9"),
        Node::new(10, Some(9), "node 10"),
    ];

    let node_list = NodeList {
        list: vec_nodes.clone(),
    };

    let node_state = NodeState {
        nodes: create_rc_signal(node_list.clone()),
    };
    let node_context = provide_context(cx, node_state);
    let root_nodes = node_context.nodes.get().get_root_nodes();

    let rnodes = create_signal(cx, root_nodes.clone());

    view! { cx,
            div(class="py-4"){
                div(class="container-sm"){
                div(class="row align-items-center"){
                    div(class="col"){
                        div (class="card", style="width: 18rem;") {
                            ul (class="list-group list-group-flush") {
                                Keyed(
                                    iterable=rnodes,
                                    view= move |cx, x|
                                        {
                                            view! { cx,
                                                    TreeNode(x)
                                            }

                                        },
                                    key=|x| x.id,
                                )

                            }
                        }
                    }

                }

            }
            }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
