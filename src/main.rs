use swayipc::{Connection, EventType, Fallible, NodeType};

// On window event, check whether there is only one window in the current workspace
// If there is, set the horizontal gaps to 1/6th of the available space
// Else, set the horizontal gaps to zero
fn main() -> Fallible<()> {
    for _ in Connection::new()?.subscribe([EventType::Window])? {
        let (focused_output, current_workspace) = focused_output_and_current_workspace().unwrap();
        let w_count = workspace_window_count(focused_output.clone(), current_workspace.clone())?;
        let (width, _) = get_output_dimensions(focused_output.clone()).unwrap();
        dbg!(
            "active output: {}\ncurrent workspace: {}\nwindow count: {}\noutput width: {}",
            focused_output.clone(),
            current_workspace.clone(),
            w_count,
            width
        );

        if w_count == 1 {
            Connection::new()?.run_command(format!(
                "gaps horizontal current set {}",
                (width as f32 / 6.0).floor()
            ))?;
        } else {
            Connection::new()?.run_command("gaps horizontal current set 0")?;
        }
    }
    Ok(())
}

// Return the number of windows in the provided output and workspace
fn workspace_window_count(output: String, workspace: String) -> Fallible<usize> {
    Ok(Connection::new()?
        .get_tree()?
        .nodes
        .iter()
        .find(|node| node.node_type == NodeType::Output && node.name == Some(output.to_string()))
        .unwrap()
        .nodes
        .iter()
        .find(|node| {
            node.node_type == NodeType::Workspace && node.name == Some(workspace.to_string())
        })
        .unwrap()
        .nodes
        .len())
}

// Get the currently focused output and workspace
fn focused_output_and_current_workspace() -> Fallible<(String, String)> {
    Ok(Connection::new()?
        .get_outputs()?
        .iter()
        .find(|output| output.focused)
        .map(|output| {
            (
                output.name.clone(),
                output.current_workspace.as_ref().unwrap().clone(),
            )
        })
        .unwrap())
}

// Get the dimensions of the named output
fn get_output_dimensions(output: String) -> Fallible<(i32, i32)> {
    Ok(Connection::new()?
        .get_outputs()?
        .iter()
        .find(|o| o.name == output)
        .map(|o| {
            let mode = o.current_mode.unwrap();
            (mode.width, mode.height)
        })
        .unwrap())
}
