
extern crate dbus;

use dbus::*;
use dbus::tree::*;

use std::sync::Arc;
use std::sync::mpsc;

static SEARCH_SERVICE_INTERFACE: &'static str = "org.freedesktop.SearchService";
static SEARCH_SERVICE_PATH: &'static str = "/org/freedesktop/SearchService";
static SEARCH_CONTEXT_INTERFACE: &'static str = "org.freedesktop.SearchContext";
static SEARCH_CONTEXT_PATH: &'static str = "/org/freedesktop/SearchContext";

#[derive(Debug)]
struct SearchService {
    contexts: Vec<Path<'static>>
}

impl SearchService {
    pub fn new() -> Self {
        SearchService {
            contexts: vec![],
        }
    }
}

impl Default for SearchService {
    fn default() -> Self {
        SearchService {
            contexts: vec![],
        }
    }
}

#[derive(Debug)]
struct SearchContext {
    context: String,
}

impl SearchContext {
    pub fn new() -> Self {
        SearchContext {
            context: String::new(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct ServiceData;
impl tree::DataType for ServiceData {
    type Tree = ();
    type ObjectPath = Option<Arc<SearchContext>>;
    type Property = ();
    type Interface = Arc<SearchService>;
    type Method = ();
    type Signal = ();
}

struct MessageHandler<M: MethodType<D>, D: DataType> {
    tree: Tree<M, D>,
    connection: Arc<Connection>,
    receiver: mpsc::Receiver<tree::ObjectPath<M, D>>,
}

impl<M: MethodType<D>, D: DataType> MessageHandler<M, D> {
    pub fn new(tree: Tree<M, D>, connection: Arc<Connection>, receiver: mpsc::Receiver<tree::ObjectPath<M, D>>) -> Self {
        MessageHandler {
            tree: tree,
            connection: connection,
            receiver: receiver
        }
    }
}

impl<M: MethodType<D>, D: DataType> dbus::MsgHandler for MessageHandler<M, D> {
    fn handler_type(&self) -> dbus::MsgHandlerType {
        self.tree.handler_type()
    }

    fn handle_msg(&mut self, msg: &Message) -> Option<MsgHandlerResult> {
        self.tree.handle_msg(msg).map(|mut r| {
            if let Ok(path) = self.receiver.try_recv() {
                let msg = if self.connection.register_object_path(path.get_name()).is_ok() {
                    let object_path = path.get_name().to_owned();
                    self.tree.insert(path);
                    msg.method_return().append1(object_path)
                } else {
                    msg.method_error(&MethodErr::failed(&"failed"))
                };

                r.reply.push(msg);
            }

            r
        })
    }
}

fn create_context_interface(service: Arc<SearchService>) -> tree::Interface<MTFn<ServiceData>, ServiceData> {
    let f = tree::Factory::new_fn();

    f.interface(SEARCH_CONTEXT_INTERFACE, service)
}

fn create_search_interface(service: Arc<SearchService>, path_tx: mpsc::Sender<tree::ObjectPath<MTFn<ServiceData>, ServiceData>>) -> tree::Interface<MTFn<ServiceData>, ServiceData> {
    let f = tree::Factory::new_fn();

    f.interface(SEARCH_SERVICE_INTERFACE, service)
        .add_m(f.method("CreateContext", (), move |m| {
            let service: &Arc<SearchService> = m.iface.get_data();

            let name: &str = m.msg.read1()?;
            println!("{}", name);

            // add new context
            let context = Arc::new(SearchContext::new());
            let f = tree::Factory::new_fn();
            let inter = create_context_interface(service.clone());
            let path = format!("{}/{}", SEARCH_CONTEXT_PATH, name);
            path_tx.send(f.object_path(path, Some(context)).introspectable().add(inter)).unwrap();

            Ok(vec![])
        }).inarg::<&str, _>("context").outarg::<&Path, _>("path")
        )
}

fn main() {
    let (path_tx, path_rx) = mpsc::channel();

    let service = Arc::new(SearchService::new());
    let interface = create_search_interface(service, path_tx);

    let f = tree::Factory::new_fn();
    let tree = f.tree(()).add(f.object_path(SEARCH_SERVICE_PATH, None).introspectable().add(interface));

    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name(SEARCH_SERVICE_INTERFACE, 0).unwrap();
    tree.set_registered(&c, true).unwrap();

    let c = Arc::new(c);
    c.add_handler(MessageHandler::new(tree, c.clone(), path_rx));

    loop {
        c.incoming(1000).next();
    }
}
