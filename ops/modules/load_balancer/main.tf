resource "google_compute_region_network_endpoint_group" "main" {
  name                  = "${var.app_name}-group"
  network_endpoint_type = "SERVERLESS"
  region                = var.region
  cloud_run {
    service = var.cloud_run_service_name
  }
}

resource "google_compute_security_policy" "ip_restrict" {
  name = "${var.app_name}-ip-restrict"

  rule {
    priority = 1000
    match {
      versioned_expr = "SRC_IPS_V1"
      config {
        src_ip_ranges = var.lb_allow_ips
      }
    }
    action = "allow"
  }

  rule {
    priority = 2147483647
    match {
      versioned_expr = "SRC_IPS_V1"
      config {
        src_ip_ranges = ["*"]
      }
    }
    action = "deny(403)"
  }
}

resource "google_compute_backend_service" "main" {
  name                  = "${var.app_name}-service"
  load_balancing_scheme = "EXTERNAL"
  protocol              = "HTTP"
  backend {
    group = google_compute_region_network_endpoint_group.main.id
  }
  security_policy = google_compute_security_policy.ip_restrict.id
}

resource "google_compute_url_map" "main" {
  name            = "${var.app_name}-url-map"
  default_service = google_compute_backend_service.main.id
}

resource "google_compute_target_http_proxy" "main" {
  name    = "${var.app_name}-proxy"
  url_map = google_compute_url_map.main.self_link
}

resource "google_compute_global_address" "main" {
  name = "${var.app_name}-address"
}

resource "google_compute_global_forwarding_rule" "main" {
  name       = "${var.app_name}-rule"
  target     = google_compute_target_http_proxy.main.self_link
  port_range = "80"
  ip_address = google_compute_global_address.main.address
}
