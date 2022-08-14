use std::{borrow::Cow, collections::HashMap, time::Duration};

use crate::types::{DataType, ValueType};
use eframe::egui::{self, DragValue, TextStyle};
use egui_node_graph::*;
use rodio::source::Zero;

use super::{types::{NodeDefinitions, SoundNode, InputValueConfig}, DEFAULT_SAMPLE_RATE};

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[derive(Clone)]
pub struct NodeData {
    pub name: String,
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
pub struct SoundGraphState {
    pub active_node: Option<NodeId>,
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<SoundGraphState> for DataType {
    fn data_type_color(&self, _user_state: &SoundGraphState) -> egui::Color32 {
        match self {
            DataType::Duration => egui::Color32::from_rgb(38, 109, 211),
            DataType::Float => egui::Color32::from_rgb(238, 207, 109),
            DataType::AudioSource => egui::Color32::from_rgb(100, 100, 100),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            DataType::Duration => Cow::Borrowed("Duration"),
            DataType::Float => Cow::Borrowed("Float"),
            DataType::AudioSource => Cow::Borrowed("AudioSource"),
        }
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
#[derive(Clone, Debug)]
pub struct NodeDefinitionUi(pub SoundNode);
impl NodeTemplateTrait for NodeDefinitionUi {
    type NodeData = NodeData;
    type DataType = DataType;
    type ValueType = ValueType;

    fn node_finder_label(&self) -> &str {
        &self.0.name.as_str()
    }

    fn node_graph_label(&self) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label().into()
    }

    fn user_data(&self) -> Self::NodeData {
        NodeData {
            name: self.0.name.clone(),
        }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.

        for input in self.0.inputs {
            graph.add_input_param(
                node_id,
                input.name.clone(),
                input.data_type,
                match input.value {
                    InputValueConfig::AudioSource => {ValueType::AudioSource { value: Box::new(Zero::new(1,DEFAULT_SAMPLE_RATE)) }}
                    InputValueConfig::Float { value } => ValueType::Float { value },
                    InputValueConfig::Duration { value } => ValueType::Duration { value: Duration::from_secs_f32(value) }
                },
                input.kind,
                true,
            );
        }
        for output in self.0.outputs{
            graph.add_output_param(node_id, output.name.clone(), output.data_type);
        }
    }
}


pub struct NodeDefinitionsUi<'a>(&'a NodeDefinitions);
impl<'a> NodeTemplateIter for NodeDefinitionsUi<'a> {
    type Item = NodeDefinitionUi;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0 .0.values().cloned().map(NodeDefinitionUi).collect()
    }
}

impl WidgetValueTrait for ValueType {
    type Response = MyResponse;
    fn value_widget(&mut self, param_name: &str, ui: &mut egui::Ui) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            ValueType::Float { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
            ValueType::Duration { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(&mut value.as_secs_f32()));
                });
            }
            ValueType::AudioSource { value } => {
                ui.label(param_name);
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

impl UserResponseTrait for MyResponse {}
impl NodeDataTrait for NodeData {
    type Response = MyResponse;
    type UserState = SoundGraphState;
    type DataType = DataType;
    type ValueType = ValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<NodeData, DataType, ValueType>,
        user_state: &Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, NodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
            }
        }

        responses
    }
}

type MyGraph = Graph<NodeData, DataType, ValueType>;
type MyEditorState =
    GraphEditorState<NodeData, DataType, ValueType, NodeDefinitionUi, SoundGraphState>;

pub struct NodeGraphExample {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    pub state: MyEditorState,
}

impl eframe::App for NodeGraphExample {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        draw_node_graph(ctx,&mut self.state,node_definitions)
    }
}

impl Default for NodeGraphExample {
    fn default() -> Self {
        Self {
            state: MyEditorState::new(1.0, SoundGraphState::default()),
        }
    }
}

pub fn draw_node_graph(ctx: &egui::Context, state: &mut MyEditorState, defs: &NodeDefinitions) {
    egui::TopBottomPanel::top("top").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });
    });
    let graph_response = egui::CentralPanel::default()
        .show(ctx, |ui| {
            state.draw_graph_editor(ui, NodeDefinitionsUi(defs))
        })
        .inner;
    for node_response in graph_response.node_responses {
        // Here, we ignore all other graph events. But you may find
        // some use for them. For example, by playing a sound when a new
        // connection is created
        if let NodeResponse::User(user_event) = node_response {
            match user_event {
                MyResponse::SetActiveNode(node) => {
                    state.user_state.active_node = Some(node)
                }
                MyResponse::ClearActiveNode => state.user_state.active_node = None,
            }
        }
    }
    
    if let Some(node) = state.user_state.active_node {
        if state.graph.nodes.contains_key(node) {
            let text = match evaluate_node(&state.graph, node, &mut HashMap::new()) {
                Ok(value) => format!("The result is: {:?}", value),
                Err(err) => format!("Execution error: {}", err),
            };
            ctx.debug_painter().text(
                egui::pos2(10.0, 35.0),
                egui::Align2::LEFT_TOP,
                text,
                TextStyle::Button.resolve(&ctx.style()),
                egui::Color32::WHITE,
            );
        } else {
            state.user_state.active_node = None;
        }
    }
}

type OutputsCache = HashMap<OutputId, ValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<ValueType> {
    // To solve a similar problem as creating node types above, we define an
    // Evaluator as a convenience. It may be overkill for this small example,
    // but something like this makes the code much more readable when the
    // number of nodes starts growing.

    struct Evaluator<'a> {
        graph: &'a MyGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    }
    impl<'a> Evaluator<'a> {
        fn new(graph: &'a MyGraph, outputs_cache: &'a mut OutputsCache, node_id: NodeId) -> Self {
            Self {
                graph,
                outputs_cache,
                node_id,
            }
        }
        fn evaluate_input(&mut self, name: &str) -> anyhow::Result<ValueType> {
            // Calling `evaluate_input` recursively evaluates other nodes in the
            // graph until the input value for a paramater has been computed.
            evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
        }
        fn populate_output(
            &mut self,
            name: &str,
            value: ValueType,
        ) -> anyhow::Result<ValueType> {
            // After computing an output, we don't just return it, but we also
            // populate the outputs cache with it. This ensures the evaluation
            // only ever computes an output once.
            //
            // The return value of the function is the "final" output of the
            // node, the thing we want to get from the evaluation. The example
            // would be slightly more contrived when we had multiple output
            // values, as we would need to choose which of the outputs is the
            // one we want to return. Other outputs could be used as
            // intermediate values.
            //
            // Note that this is just one possible semantic interpretation of
            // the graphs, you can come up with your own evaluation semantics!
            populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
        }
        fn input_vector(&mut self, name: &str) -> anyhow::Result<egui::Vec2> {
            self.evaluate_input(name)?.try_to_vec2()
        }
        fn input_scalar(&mut self, name: &str) -> anyhow::Result<f32> {
            self.evaluate_input(name)?.try_to_scalar()
        }
        fn output_vector(&mut self, name: &str, value: egui::Vec2) -> anyhow::Result<ValueType> {
            self.populate_output(name, ValueType::Vec2 { value })
        }
        fn output_scalar(&mut self, name: &str, value: f32) -> anyhow::Result<ValueType> {
            self.populate_output(name, ValueType::Scalar { value })
        }
    }

    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        MyNodeTemplate::AddScalar => {
            let a = evaluator.input_scalar("A")?;
            let b = evaluator.input_scalar("B")?;
            evaluator.output_scalar("out", a + b)
        }
        MyNodeTemplate::SubtractScalar => {
            let a = evaluator.input_scalar("A")?;
            let b = evaluator.input_scalar("B")?;
            evaluator.output_scalar("out", a - b)
        }
        MyNodeTemplate::VectorTimesScalar => {
            let scalar = evaluator.input_scalar("scalar")?;
            let vector = evaluator.input_vector("vector")?;
            evaluator.output_vector("out", vector * scalar)
        }
        MyNodeTemplate::AddVector => {
            let v1 = evaluator.input_vector("v1")?;
            let v2 = evaluator.input_vector("v2")?;
            evaluator.output_vector("out", v1 + v2)
        }
        MyNodeTemplate::SubtractVector => {
            let v1 = evaluator.input_vector("v1")?;
            let v2 = evaluator.input_vector("v2")?;
            evaluator.output_vector("out", v1 - v2)
        }
        MyNodeTemplate::MakeVector => {
            let x = evaluator.input_scalar("x")?;
            let y = evaluator.input_scalar("y")?;
            evaluator.output_vector("out", egui::vec2(x, y))
        }
        MyNodeTemplate::MakeScalar => {
            let value = evaluator.input_scalar("value")?;
            evaluator.output_scalar("out", value)
        }
    }
}

fn populate_output(
    graph: &MyGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: ValueType,
) -> anyhow::Result<ValueType> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value);
    Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
    graph: &MyGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<ValueType> {
    let input_id = graph[node_id].get_input(param_name)?;

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok(*other_value)
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

            // Now that we know the value is cached, return it
            Ok(*outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated"))
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value)
    }
}
