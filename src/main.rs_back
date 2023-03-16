use gloo::console::log;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::*;
use web_sys::{DataTransfer, Event};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
}

impl Node {
    fn new(id: i32, parent_id: Option<i32>, name: &str) -> Node {
        Node {
            id,
            parent_id,
            name: name.to_owned(),
        }
    }

    fn has_child(&self, nodes: &[Node]) -> bool {
        nodes.iter().any(|n| n.parent_id == Some(self.id))
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

#[derive(Debug, Default, Clone)]
pub struct NodeState {
    pub nodes: RcSignal<NodeList>,
}

#[component(inline_props)]
fn NestedNode<'a, G: Html>(cx: Scope<'a>, n: Node, items: &'a Signal<Vec<Node>>) -> View<G> {
    // modif
    let node_ref = create_node_ref::<G>(cx);
    let node = n.clone();
    let nd = n.clone();
    let nodes = items.get();
    let top_children = n
        .get_immediate_children(&nodes)
        .into_iter()
        .cloned()
        .collect();

    let children_signal = create_signal(cx, top_children);

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

    let c_item = create_signal(cx, nd);
    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        if e.type_().contains("dragstart") {
            data_transf.set_effect_allowed("move");
            data_transf
                .set_data("text/html", &c_item.get().id.to_string())
                .unwrap();

            log!(format!("Transfer {:?}", &c_item.get()));
        }
        dom.set_attribute("style", "opacity: 0.2");
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 1");
        log!(format!("{:?}", e.type_()));
    };
    let handle_dragenter = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.add_class("bg-primary");
        log!(format!("{:?}", e.type_()));
    };

    let handle_dragover = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        e.prevent_default();
        dom.add_class("bg-info");
    };

    let handle_dragleave = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.remove_class("bg-info");
        log!(format!("{:?}", e));
    };

    let handle_drop = move |e: Event| {
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        let data = data_transf.get_data("text/html").unwrap();

        log!(format!("{:?}", data.clone()));
        log!(format!("{:?}", &c_item.get()));

        let mut items = items.modify();
        let dragged_index = items
            .iter()
            .position(|i| i.id == data.parse::<i32>().unwrap())
            .unwrap();
        let target_index = items.iter().position(|i| i.id == c_item.get().id).unwrap();
        items.swap(dragged_index, target_index);
    };

    view! {cx,  i(on:click=toggle, class=class()) (node.name)
     (if *toggle_state.get() {

         view! { cx,
            ul(class="list-group") {
                Keyed(
                    iterable=children_signal,
                    view= move |cx, x| view! { cx,
                       li(class="list-group-item", ref=node_ref, draggable=true, on:dragstart=handle_dragstart, on:dragend=handle_dragend, on:dragenter=handle_dragenter, on:dragover=handle_dragover, on:dragleave=handle_dragleave, on:drop=handle_drop) {NestedNode(n=x, items=items)}
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

#[component(inline_props)]
fn TreeNode<'a, G: Html>(cx: Scope<'a>, n: Node, items: &'a Signal<Vec<Node>>) -> View<G> {
    /*
    let all_nodes = use_context::<NodeState>(cx);
    let nodes = all_nodes.nodes.get().as_ref().clone().list;
    let x_child = n.clone();
    let n_haschild = x_child.has_child(&nodes);
    */

    let has_child = n.has_child(&items.get());
    view! { cx,
            li(class="list-group-item") {
                    (if has_child {
                        let nx = n.clone();


                   view!(cx, NestedNode(n=nx, items=items))
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

    let vnodes = create_signal(cx, vec_nodes);
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
                                                    TreeNode(n=x, items=vnodes)

                                                    /*
                                                    li(class="list-group-item") {
                                                        (if n_haschild {
                                                            let nx = n.clone();
                                                            view!(cx, NestedNode(nx))
                                                        } else {
                                                            let nx = n.clone();
                                                            view! { cx,  (nx.name)}
                                                        })
                                                    }
                                                    */
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
