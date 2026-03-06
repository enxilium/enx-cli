use crate::config;
use crate::output;
use dialoguer::Confirm;

pub fn run(name: &str) -> anyhow::Result<()> {
    let registry_path = config::registry_path()?;
    let mut registry = config::registry::Registry::load_from_file(&registry_path)?;

    if registry.find_path_by_name(name).is_none() {
        return Err(anyhow::anyhow!(
            "No project with this name found in the registry."
        ));
    }

    let project_path = registry.find_path_by_name(name).unwrap().clone();

    registry.remove_project(name);
    registry.save_to_file(&registry_path)?;

    output::success(&format!("Project '{}' removed from the registry.", name));

    if project_path.exists() {
        let should_delete = Confirm::new()
            .with_prompt("Do you also want to delete the project directory?")
            .default(false)
            .interact()?;

        if should_delete {
            std::fs::remove_dir_all(&project_path)?;
            output::success(&format!(
                "Project directory '{}' deleted.",
                project_path.display()
            ));
        } else {
            output::warning(&format!(
                "Project directory '{}' was not deleted. You may want to delete it manually if you no longer need it.",
                project_path.display()
            ));
        }
    }

    Ok(())
}
