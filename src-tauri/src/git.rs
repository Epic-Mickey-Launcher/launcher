use std::path::PathBuf;

pub fn clone(url: &String, destination: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    git2::Repository::clone_recurse(url, destination)?;
    Ok(())
}

pub fn init_repository(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::init(path)?;
    let seconds_since_epoch = chrono::Local::now().to_utc().timestamp();
    let time = git2::Time::new(seconds_since_epoch, 0);
    let signature = git2::Signature::new("EML", "epicmickeylauncher@gmail.com", &time)?;
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    index.write()?;
    let tree = repo.find_tree(oid)?;
    repo.commit(Some("HEAD"), &signature, &signature, "Initial", &tree, &[])?;
    Ok(())
}

pub fn update_mod(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::open(path)?;
    let seconds_since_epoch = chrono::Local::now().to_utc().timestamp();
    let time = git2::Time::new(seconds_since_epoch, 0);
    let signature = git2::Signature::new("EML", "epicmickeylauncher@gmail.com", &time)?;
    let mut index = repo.index()?;
    index
        .add_all(&["."], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    let oid = index.write_tree()?;
    index.write()?;
    let tree = repo.find_tree(oid)?;
    let parent_commit = repo.head().unwrap().peel_to_commit()?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Update",
        &tree,
        &[&parent_commit],
    )?;

    let remotes = repo.remotes()?;

    if !remotes.is_empty() {
        let remote_url = remotes.get(0).unwrap();
        let mut remote = repo.find_remote(remote_url)?;
        remote.push(&["origin"], None)?;
    }

    Ok(())
}
