use binder::service::ServiceManager;

const PACKAGE_MANAGER: &'static str = "android.content.pm.IPackageManager";
fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    log::error!("Starting the getsvc example");

    let svc = ServiceManager::current()
        .query(&PACKAGE_MANAGER).unwrap();

}