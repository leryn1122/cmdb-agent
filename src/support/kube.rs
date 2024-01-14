use std::path::Path;

const KUBERNETES_SERVICEACCOUNT_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount";

pub fn is_under_kubernetes() -> bool {
  Path::new(KUBERNETES_SERVICEACCOUNT_PATH).exists()
}
