use std::process::Command;

pub fn apt() -> Result<(), String> {
    change_apt_source()
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

pub fn git() -> Result<(), String> {
    let output = Command::new("/usr/bin/apt")
                    .arg("install")
                    .arg("git")
                    .output()
                    .expect("Apt installing git failed");
    
    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(())
    }
}

pub fn curl() -> Result<(), String> {
    let output = Command::new("/usr/bin/apt")
                    .arg("install")
                    .arg("curl")
                    .output()
                    .expect("Apt installing git failed");
    
    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(())
    }
}