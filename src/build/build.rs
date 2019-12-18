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

pub fn shell() -> Result<(), String> {
    Ok(())
}

pub fn ohmyzsh() -> Result<(), String> {
    let output = Command::new("/bin/sh")
                    .arg("-c \"$(curl -fsSL \
                        https://raw.githubusercontent.com\
                        /ohmyzsh/ohmyzsh/master/tools/install.sh)\"")
                    .output()
                    .expect("failed");
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string())
    }

    Ok(())
}
