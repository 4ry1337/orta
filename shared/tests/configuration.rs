use shared::configuration::get_configuration;

pub fn configuration_test() {
    let configuration = get_configuration().unwrap();

    assert_eq!(configuration.api_server.port, 5000);
}
