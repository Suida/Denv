use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

pub fn apt_install(app_name: &str) -> Result<(), String> {
    let app_name = if app_name.ends_with("<apt-install>") {
        &app_name[0..(app_name.len() - 13)]
    } else {
        &app_name
    };

    let output = Command::new("/usr/bin/apt")
                    .arg("install")
                    .arg(app_name)
                    .output()
                    .expect("Apt installing git failed");
    
    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(())
    }
}

pub fn change_apt_source() -> Result<(), String> {
    let mut outputs = Vec::with_capacity(3);

    outputs.push(Command::new("/bin/mv")
                    .arg("/etc/apt/sources.list")
                    .arg("/etc/apt/sources.list.bkp")
                    .output()
                    .expect("failed"));

    outputs.push(Command::new("/bin/cp")
                    .arg("modules/sources/tsinghua.apt.sources.list")
                    .arg("/etc/apt/sources.list")
                    .output()
                    .expect("failed"));

    outputs.push(Command::new("apt")
                    .arg("update")
                    .output()
                    .expect("failed"));

    for output in outputs.iter() {
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    Ok(())
}

pub fn shell(proxy: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
                    .append(true)
                    .open("~/.zshrc")
                    .expect("Open ~/.zshrc failed");

    file.write_all(format!("export http_proxy={}", proxy).as_bytes()).expect("HTTP proxy set failed");
    file.write_all(format!("export https_proxy={}", proxy).as_bytes()).expect("HTTP proxy set failed");

    let output = Command::new("zsh")
                    .output()
                    .expect("Restart zsh failed");
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string())
    }

    Ok(())
}

pub fn ohmyzsh() -> Result<(), String> {
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c \"$(curl -fsSL \
            https://raw.githubusercontent.com\
            /ohmyzsh/ohmyzsh/master/tools/install.sh)\"");

    let status = cmd.status().expect("Ohmyzsh downloading failed.");
    
    if !status.success() {
        return Err("Ohmyzsh downloading failed.".to_string())
    }

    Ok(())
}

pub fn pyenv() -> Result<(), String> {
    let output = Command::new("git")
                    .arg("clone")
                    .arg("http://github.com/pyenv/pyenv")
                    .arg("~/.pyenv")
                    .output()
                    .expect("Git clone to pyenv failed.");

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
        
    let mut file = OpenOptions::new()
                    .append(true)
                    .open("~/.zshrc")
                    .expect("Open ~/.zshrc failed");

    file.write_all("export PYENV_ROOT=\"$HOME/.pyenv\"".as_bytes()).expect("Write .zshrc failed");
    file.write_all("export PATH=\"$PYENV_ROOT/bin:$PATH\"".as_bytes()).expect("Write .zshrc failed");
    file.write_all("if command -v pyenv 1>/dev/null 2>&1; then\n  eval \"$(pyenv init -)\"\nfi" .as_bytes())
        .expect("Write .zshrc failed");

     let output = Command::new("exec")
                    .arg("\"SHELL\"")
                    .output()
                    .expect("Git clone to pyenv failed.");

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string())
    }

    Ok(())
}
