// Provide functionality over seperate methods (bad)
// Collect all of them in one JS object, so dont bother us with context handle (good)

use std::sync::{LazyLock, MutexGuard};

use sharded_slab::{Entry, Slab};

use speedy2d::GLRenderer;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, window};

use pree::growth::Growable;
use pree::render::Viewport;
use pree::tree::{SharedTreeContext, Tree, TreeContext, TreeContextHandle, TreeTheme};

mod producers;

use producers::ExampleTreeProducer;

static TREE_CONTEXTS: LazyLock<Slab<SharedTreeContext>> = LazyLock::new(|| Slab::new());

fn get_tree_context_entry(
    handle: TreeContextHandle,
) -> Result<Entry<'static, SharedTreeContext>, JsError> {
    TREE_CONTEXTS
        .get(handle)
        .ok_or(JsError::new("Invalid Tree Context handle!"))
}

fn lock_tree_context<'e>(
    entry: &'e Entry<'static, SharedTreeContext>,
) -> Result<MutexGuard<'e, TreeContext>, JsError> {
    entry
        .lock()
        .map_err(|_| JsError::new("Tree Context's mutex is poisoned!"))
}

fn get_canvas_size(id: &str) -> Result<(u32, u32), JsError> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let element = document
        .get_element_by_id(id)
        .ok_or(JsError::new("There is no canvas with this name!"))?;
    let canvas: HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| JsError::new("This element is not a canvas!"))?;

    let width = canvas.width();
    let height = canvas.height();

    Ok((width, height))
}

#[wasm_bindgen]
pub fn crate_tree(
    canvas_id: &str,
    x: f32,
    y: f32,
    length: f32,
    angle: f32,
    thickness: f32,
    depth: u8,
    theme: &str,
) -> Result<Option<TreeContextHandle>, JsError> {
    let viewport_sizes = get_canvas_size(canvas_id)?;
    let viewport = Viewport::from(viewport_sizes);

    let renderer = GLRenderer::new_for_web_canvas_by_id(viewport_sizes, canvas_id)?;

    let tree = Tree::new(
        (x, y).into(),
        length,
        angle,
        thickness,
        depth,
        &ExampleTreeProducer::new(),
    );
    let theme = TreeTheme::try_from(theme)?;

    let tree_context = TreeContext::new(renderer, viewport, tree, theme);
    let handle = TREE_CONTEXTS.insert(tree_context.into());

    Ok(handle)
}

#[wasm_bindgen]
pub fn render_tree(handle: TreeContextHandle) -> Result<(), JsError> {
    let tree_context_entry = get_tree_context_entry(handle)?;
    let mut tree_context = lock_tree_context(&tree_context_entry)?;

    tree_context.render();
    Ok(())
}

#[wasm_bindgen]
pub fn grow_tree(handle: TreeContextHandle) -> Result<(), JsError> {
    let tree_context_entry = get_tree_context_entry(handle)?;
    let mut tree_context = lock_tree_context(&tree_context_entry)?;

    tree_context.grow();
    Ok(())
}

#[wasm_bindgen]
pub fn is_tree_fully_grown(handle: TreeContextHandle) -> Result<bool, JsError> {
    let tree_context_entry = get_tree_context_entry(handle)?;
    let tree_context = lock_tree_context(&tree_context_entry)?;

    Ok(tree_context.get_tree().is_fully_grown())
}
