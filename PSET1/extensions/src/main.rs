use rust50::input;

fn main() {
    let ext = ["gif", "jpg", "jpeg", "png", "pdf", "txt", "zip"];
    let ty = [
        "image/gif",
        "image/jpeg",
        "image/jpeg",
        "image/png",
        "application/pdf",
        "text/plain",
        "application/zip",
    ];
    let inp: String = input("File: ");
    let extn = inp.trim().split(".").nth(1).unwrap();
    if ext.contains(&extn) {
        for i in 0..ext.len() {
            if ext[i] == extn {
                println!("{}", ty[i])
            }
        }
    } else {
        println!("application/octet-stream")
    }
}
